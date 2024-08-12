use chrono::Duration;
use mlc_common::effect::player::{EffectPlayerMsg, EffectPlayerRequest};
use pollster::FutureExt;
use rocket::fairing::AdHoc;
use rocket::futures::{SinkExt, StreamExt};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{self, Sender};
use rocket::{get, Shutdown, State};
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_ws::stream::DuplexStream;
use rocket_ws::WebSocket;

use mlc_common::effect::rest::{EffectHandlerRequest, EffectHandlerResponse};
use mlc_common::effect::{Effect, EffectId};
use mlc_common::Info;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::data_serving::ProjectGuard;
use crate::project::ProjectHandle;
use crate::runtime::effects::player::EffectPlayerUpdate;
use crate::{module::Module, send};

use self::player::{startup_effect_player, EffectPlayerCmd, EffectPlayerHandle};

use super::{decode_msg, RuntimeData};

mod baking;
pub mod player;

pub struct EffectModule;

impl Module for EffectModule {
    fn setup(
        &self,
        app: rocket::Rocket<rocket::Build>,
        spec: &mut OpenApi,
    ) -> rocket::Rocket<rocket::Build> {
        let (effect_handler_tx, effect_handler_rx) =
            broadcast::channel::<InterEffectHandlerMsg>(512);

        let effect_player = startup_effect_player(
            app.state::<ProjectHandle>().unwrap().clone(),
            app.state::<RuntimeData>().unwrap().clone(),
        )
        .block_on();

        // let tx = startup_effect_player(
        //     app.state::<RuntimeData>().unwrap().clone(),
        //     app.state::<ProjectHandle>().unwrap().clone(),
        //     baking_tx.clone(),
        // );

        let (routes, s) =
            openapi_get_routes_spec![get_effect_handler, get_effect_list, get_effect_player];
        merge_specs(spec, &"/effects".to_string(), &s).expect("Merging OpenApi failed");

        app.manage(effect_player)
            .manage(effect_handler_rx)
            .manage(effect_handler_tx)
            .attach(AdHoc::on_shutdown("Shutdown EffectPlayer", |a| {
                Box::pin(async move {
                    let _ = a
                        .state::<Sender<EffectPlayerCmd>>()
                        .unwrap()
                        .send(EffectPlayerCmd::StopPlayer);
                })
            }))
            .mount("/effects", routes)
    }
}

/// # Get Effects
/// Returns a List of Tuples ([`String`], [`uuid::Uuid`]) containing the projects Effects names and ids.
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Effects")]
#[get("/get")]
async fn get_effect_list(
    project: &State<ProjectHandle>,
    _g: ProjectGuard,
) -> Json<Vec<(String, uuid::Uuid)>> {
    let p = project.lock().await;
    Json(
        p.effects
            .iter()
            .map(|e| (e.name.to_string(), e.id))
            .collect(),
    )
}

/// Serves the purpose to send messages between effectHandlers
#[derive(Clone, Debug)]
pub enum InterEffectHandlerMsg {
    Updated { id: uuid::Uuid },
}

/// # Effect Handler
/// Opens a WebSocket connection to control effect creation and playback.
///
/// See [EffectHandlerRequest]
///
/// [Guarded][ProjectGuard]
#[openapi(tag = "Effects")]
#[get("/effectHandler")]
async fn get_effect_handler<'a>(
    ws: WebSocket,
    mut shutdown: Shutdown,
    info: &'a State<Sender<Info>>,
    tx: &'a State<EffectPlayerHandle>,
    effect_handler_tx: &'a State<Sender<InterEffectHandlerMsg>>,
    project: &'a State<ProjectHandle>,
    _g: ProjectGuard,
) -> rocket_ws::Channel<'a> {
    let mut rx = effect_handler_tx.subscribe();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    Some(msg) = stream.next() => {
                        if let Ok(msg) = msg {
                            if let Some(req) = decode_msg::<EffectHandlerRequest>(&msg){

                                let send_info = matches!(req, EffectHandlerRequest::Create {..});

                                handle_msg(&mut stream, req, tx.inner().clone(), effect_handler_tx, project).await;
                                if send_info {
                                    send!(info, Info::EffectListChanged);
                                }
                            }
                        }
                    }
                    Ok(msg) = rx.recv() => {
                        let m = match msg {
                            InterEffectHandlerMsg::Updated {id} => {
                                Some(EffectHandlerResponse::EffectUpdated {
                                    id,
                                })
                            }
                        };

                        if let Some(m) = m {
                            let _ = stream.send(make_msg(&m)).await;
                        }
                    }
                    _ = &mut shutdown => {
                        break;
                    }
                }
            }
            Ok(())
        })
    })
}

/// # Get Effect Player
/// Upgrades to a WebSocket connection on which communication with the Effect Player is possible.
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Effects")]
#[get("/effectPlayer")]
async fn get_effect_player(
    ws: WebSocket,
    mut shutdown: Shutdown,
    tx: &State<EffectPlayerHandle>,
) -> rocket_ws::Channel {
    let mut effect_player = tx.inner().clone();
    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    Ok(msg) = effect_player.update_receiver.recv() => {
                        let msg = match msg {
                            EffectPlayerUpdate::PlayingEffects(effects) => EffectPlayerMsg::PlayingEffects {
                                effects,
                            }
                        };

                        let _ = stream.send(make_msg(&msg)).await;
                    }

                    Some(msg) = stream.next() => {
                        if let Ok(msg) = msg {
                            if let Some(req) = decode_msg::<EffectPlayerRequest>(&msg){
                                match req {
                                    EffectPlayerRequest::Play { effect } => {let _ = effect_player.cmd_sender.send(EffectPlayerCmd::Play { id: effect }).await;},
                                    EffectPlayerRequest::Stop { effect } => {let _ = effect_player.cmd_sender.send(EffectPlayerCmd::Stop  { id: effect }).await;},
                                }
                            }
                        }
                    }


                    _ = &mut shutdown => {
                        break;
                    }
                }
            }
            Ok(())
        })
    })
}

async fn handle_msg(
    stream: &mut DuplexStream,
    req: EffectHandlerRequest,
    // runtime: &State<RuntimeData>,
    mut effect_player: EffectPlayerHandle,
    effect_handler_tx: &Sender<InterEffectHandlerMsg>,
    project: &ProjectHandle,
) {
    match req {
        EffectHandlerRequest::Create { name } => {
            let name = validate_effect_name(name);
            let mut p = project.lock().await;
            let id = uuid::Uuid::new_v4();
            p.effects.push(Effect {
                id,
                name: name.clone(),
                duration: Duration::seconds(5),
                tracks: vec![],
                looping: false,
            });
            let _ = stream
                .send(make_msg(&EffectHandlerResponse::EffectCreated { name, id }))
                .await;
        }
        EffectHandlerRequest::Update {
            id,
            tracks,
            looping,
            duration,
        } => {
            let mut p = project.lock().await;
            let effect = p.effects.iter_mut().find(|f| f.id == id);
            if let Some(effect) = effect {
                effect.tracks = tracks;
                effect.looping = looping;
                effect.duration = duration;
            }
            // let _ = stream
            //     .send(make_msg(&EffectHandlerResponse::EffectUpdated { id }))
            //     .await;
            effect_handler_tx
                .send(InterEffectHandlerMsg::Updated { id })
                .unwrap();
            effect_player
                .cmd_sender
                .send(EffectPlayerCmd::EffectChanged { id })
                .await
                .unwrap();
        }
        EffectHandlerRequest::Get { id } => {
            let p = project.lock().await;
            let effect = p.effects.iter().find(|e| e.id == id);
            if let Some(e) = effect {
                let _ = stream
                    .send(make_msg(&EffectHandlerResponse::Effect {
                        effect: e.clone(),
                    }))
                    .await;
            }
        }
        EffectHandlerRequest::List => {
            let p = project.lock().await;
            let effects = p.effects.iter().map(|e| (e.name.clone(), e.id)).collect();
            let _ = stream
                .send(make_msg(&EffectHandlerResponse::EffectList { effects }))
                .await;
        }
    }
}

fn validate_effect_name(name: String) -> String {
    let parts = name.split('/');
    parts.map(|p| p.trim()).fold("".to_string(), |a, p| {
        if a.is_empty() {
            a + p
        } else {
            a + "/" + p
        }
    })
}

fn make_msg<T: serde::Serialize>(t: &T) -> rocket_ws::Message {
    rocket_ws::Message::Text(serde_json::to_string(t).unwrap())
}

// struct EffectPlayerI {
//     runtime: RuntimeData,
//     project: ProjectHandle,
//     effects: HashMap<uuid::Uuid, BakedEffect>,
//     rx: Receiver<EffectPlayerAction>,
//     time: chrono::NaiveTime,
//     baking: Sender<BakingNotification>,
// }

// #[derive(Debug, Clone)]
// pub enum EffectPlayerAction {
//     Stop,
//     EffectsChanged { id: uuid::Uuid },
//     Rebake,
//     Toggle { id: uuid::Uuid },
// }

// fn startup_effect_player(
//     runtime: RuntimeData,
//     project: Project,
//     baking: Sender<BakingNotification>,
// ) -> Sender<EffectPlayerAction> {
//     let (tx, rx) = broadcast::channel(500);

//     rocket::tokio::spawn(async move {
//         let effect_player = EffectPlayerI {
//             effects: HashMap::new(),
//             project,
//             runtime,
//             rx,
//             time: chrono::Utc::now().naive_utc().time(),
//             baking,
//         };
//         effect_player.start().await;
//     });

//     tx
// }

// const EFFECT_UPDATE_FREQ: u64 = 20; //ms //TODO: Make available in settings

// impl EffectPlayerI {
//     async fn start(mut self) {
//         let mut sleep =
//             rocket::tokio::time::interval(std::time::Duration::from_millis(EFFECT_UPDATE_FREQ));
//         loop {
//             select! {
//                 _ = sleep.tick() => {
//                     self.update().await;
//                 }
//                 Ok(msg) = self.rx.recv() => {
//                     if self.handle_action(msg).await {
//                         break;
//                     }
//                 }
//             }
//         }
//     }

//     async fn handle_action(&mut self, msg: EffectPlayerAction) -> bool {
//         match msg {
//             EffectPlayerAction::Stop => return true,
//             EffectPlayerAction::Rebake => {
//                 self.bake_effects().await; //TODO: Make player not wait for baking to finish just queue and update when baking completed then also needs to run on seperate thread
//             }
//             EffectPlayerAction::EffectsChanged { id } => {
//                 self.bake_effect(id).await; //TODO: Same as above but then also needs to run on seperate thread
//             }
//             EffectPlayerAction::Toggle { id } => {
//                 if let Some(e) = self.effects.get_mut(&id) {
//                     e.toggle()
//                 }
//             }
//         }

//         false
//     }

//     async fn update(&mut self) {
//         let now = chrono::Utc::now().naive_utc().time();
//         let elapsed = now - self.time;
//         self.time = now;

//         let mut value_map = HashMap::new();

//         for e in self.effects.values_mut() {
//             if !e.running || e.max_time < Duration::milliseconds(2) {
//                 continue;
//             }

//             e.current_time += elapsed;
//             if e.current_time > e.max_time {
//                 if e.looping {
//                     while e.current_time > e.max_time {
//                         e.current_time -= e.max_time;
//                     }
//                 } else {
//                     e.running = false;
//                 }
//             }

//             for f in &e.faders {
//                 let mut value = 0;
//                 for (d, v) in f.1.iter() {
//                     if &e.current_time > d {
//                         value = *v;
//                     }
//                 }
//                 value_map.insert(*f.0, value);
//             }
//         }

//         let mut universes = vec![];
//         let mut channels = vec![];
//         let mut values = vec![];

//         for (k, v) in value_map {
//             universes.push(k.universe);
//             channels.push(k.address);
//             values.push(v);
//         }
//         if !universes.is_empty() {
//             self.runtime.set_values(universes, channels, values).await;
//         }
//     }

//     async fn bake_effects(&mut self) {
//         self.effects.clear();

//         let p = self.project.lock().await;
//         let effects = &p.effects;

//         let patched_fixtures = get_patched_fixtures_clone(&p);

//         send!(
//             self.baking,
//             BakingNotification::Started("Baking effects started!".to_string())
//         );
//         let time = Instant::now();
//         let s: Vec<_> = effects
//             .iter()
//             .map(|e| (e.id, baking::bake(e, patched_fixtures.clone())))
//             .collect();
//         let l = s.len();
//         send!(
//             self.baking,
//             BakingNotification::Misc("Baking effects running!".to_string())
//         );

//         for (id, ele) in s {
//             let baked = ele.await;
//             self.effects.insert(id, baked);
//         }

//         let duration = time.elapsed();
//         send!(
//             self.baking,
//             BakingNotification::Finished(format!("Baking {} effects completed in {}", l, duration))
//         );
//         println!("Finished Baking!");
//     }

//     async fn bake_effect(&mut self, effect: uuid::Uuid) {
//         let p = self.project.lock().await;
//         let effects = &p.effects;
//         let e = effects.iter().find(|e| e.id == effect);

//         let patched_fixtures = get_patched_fixtures_clone(&p);

//         send!(
//             self.baking,
//             BakingNotification::Started("Baking single effect!".to_string())
//         );
//         let time = Instant::now();
//         if let Some(e) = e {
//             let baked = baking::bake(e, patched_fixtures).await;
//             self.effects.insert(e.id, baked);
//         }

//         let duration = time.elapsed();
//         send!(
//             self.baking,
//             BakingNotification::Finished(format!("Baking single effect complete in {}", duration))
//         );
//         println!("Done");
//     }
// }

// fn get_patched_fixtures_clone(p: &MutexGuard<ProjectI>) -> BakedFixtureData {
//     use get_size::GetSize;
//     let patched_fixtures: Vec<_> = p
//         .universes
//         .values()
//         .flat_map(|u| u.fixtures.clone())
//         .collect();
//     println!(
//         "Debug: Patched fixture clone size for baking {} bytes",
//         patched_fixtures.get_heap_size()
//     );
//     patched_fixtures
// }
