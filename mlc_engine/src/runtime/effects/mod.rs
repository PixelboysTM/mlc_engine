use std::collections::HashMap;

use chrono::Duration;
use mlc_common::fixture::FaderAddress;
use mlc_common::Info;
use rocket::fairing::AdHoc;
use rocket::futures::lock::MutexGuard;
use rocket::futures::{SinkExt, StreamExt};
use rocket::serde::json::Json;
use rocket::time::Instant;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{self, Receiver, Sender};
use rocket::{get, routes, Shutdown, State};
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_spec};
use rocket_ws::stream::DuplexStream;
use rocket_ws::WebSocket;
use serde_with::serde_as;
use serde_with::{formats::Flexible, DurationSecondsWithFrac};

use crate::data_serving::ProjectGuard;
use crate::project::{Project, ProjectI};
use crate::runtime::effects::baking::{BakedEffect, BakedFixtureData, BakingNotification};
use crate::runtime::effects::feature_track::FeatureTrack;
use crate::runtime::effects::track_key::FaderKey;
use crate::{module::Module, send};

use super::{decode_msg, RuntimeData};

mod baking;
mod feature_track;
mod track_key;

pub struct EffectModule;

impl Module for EffectModule {
    fn setup(
        &self,
        app: rocket::Rocket<rocket::Build>,
        spec: &mut OpenApi,
    ) -> rocket::Rocket<rocket::Build> {
        let (baking_tx, baking_rx) = broadcast::channel::<BakingNotification>(512);

        let tx = startup_effect_player(
            app.state::<RuntimeData>().unwrap().clone(),
            app.state::<Project>().unwrap().clone(),
            baking_tx.clone(),
        );

        let routes = routes![
            get_effect_handler,
            get_effect_list,
            get_baking_notifications
        ];
        let s = openapi_get_spec![get_effect_list, get_baking_notifications];
        merge_specs(spec, &"/effects".to_string(), &s).expect("Merging OpenApi failed");

        app.manage(tx)
            .manage(baking_rx)
            .manage(baking_tx)
            .attach(AdHoc::on_shutdown("Shutdown EffectPlayer", |a| {
                Box::pin(async move {
                    let _ = a
                        .state::<Sender<EffectPlayerAction>>()
                        .unwrap()
                        .send(EffectPlayerAction::Stop);
                })
            }))
            .mount("/effects", routes)
    }
}

#[openapi(tag = "Effects")]
#[get("/get")]
async fn get_effect_list(
    project: &State<Project>,
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

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Effect {
    id: uuid::Uuid,
    name: String,
    looping: bool,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    duration: Duration,
    tracks: Vec<Track>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum Track {
    FaderTrack(FaderTrack),
    FeatureTrack(FeatureTrack),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct FaderTrack {
    address: FaderAddress,
    values: Vec<FaderKey>,
}

#[derive(Debug, serde::Serialize)]
#[allow(dead_code)]
pub enum EffectHandlerResponse {
    EffectCreated { name: String, id: uuid::Uuid },
    EffectUpdated { id: uuid::Uuid },
    EffectRunning { id: uuid::Uuid, running: bool },
    EffectList { effects: Vec<(String, uuid::Uuid)> },
    Effect { effect: Effect },
}

#[serde_as]
#[derive(Debug, serde::Deserialize)]
pub enum EffectHandlerRequest {
    Create {
        name: String,
    },
    Update {
        id: uuid::Uuid,
        tracks: Vec<Track>,
        looping: bool,
        #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
        duration: Duration,
    },
    Toggle {
        id: uuid::Uuid,
    },
    Get {
        id: uuid::Uuid,
    },
    List,
}

#[openapi]
#[get("/effectHandler")]
async fn get_effect_handler<'a>(
    ws: WebSocket,
    mut shutdown: Shutdown,
    info: &'a State<Sender<Info>>,
    tx: &'a State<Sender<EffectPlayerAction>>,
    project: &'a State<Project>,
    _g: ProjectGuard,
) -> rocket_ws::Channel<'a> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    Some(msg) = stream.next() => {
                        if let Ok(msg) = msg {
                            if let Some(req) = decode_msg::<EffectHandlerRequest>(&msg){

                                let send_info = matches!(req, EffectHandlerRequest::Create {..});

                                handle_msg(&mut stream, req, tx, project).await;
                                if send_info {
                                    send!(info, Info::EffectListChanged);
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

/// # Baking notifications
/// Upgrades to a WebSocket connection on which updates about baking processes
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Effects")]
#[get("/baking")]
async fn get_baking_notifications(
    ws: WebSocket,
    mut shutdown: Shutdown,
    tx: &State<Sender<BakingNotification>>,
) -> rocket_ws::Channel {
    let mut rx = tx.subscribe();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    Ok(msg) = rx.recv() => {
                        let _ = stream.send(make_msg(&msg)).await;
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
    tx: &Sender<EffectPlayerAction>,
    project: &Project,
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
            let _ = stream
                .send(make_msg(&EffectHandlerResponse::EffectUpdated { id }))
                .await;
            tx.send(EffectPlayerAction::EffectsChanged { id }).unwrap();
        }
        EffectHandlerRequest::Toggle { id } => {
            tx.send(EffectPlayerAction::Toggle { id }).unwrap();
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
            let effects = p
                .effects
                .iter()
                .map(|e| (e.name.clone(), e.id))
                .collect();
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

struct EffectPlayerI {
    runtime: RuntimeData,
    project: Project,
    effects: HashMap<uuid::Uuid, BakedEffect>,
    rx: Receiver<EffectPlayerAction>,
    time: chrono::NaiveTime,
    baking: Sender<BakingNotification>,
}

#[derive(Debug, Clone)]
pub enum EffectPlayerAction {
    Stop,
    EffectsChanged { id: uuid::Uuid },
    Rebake,
    Toggle { id: uuid::Uuid },
}

fn startup_effect_player(
    runtime: RuntimeData,
    project: Project,
    baking: Sender<BakingNotification>,
) -> Sender<EffectPlayerAction> {
    let (tx, rx) = broadcast::channel(500);

    rocket::tokio::spawn(async move {
        let effect_player = EffectPlayerI {
            effects: HashMap::new(),
            project,
            runtime,
            rx,
            time: chrono::Utc::now().naive_utc().time(),
            baking,
        };
        effect_player.start().await;
    });

    tx
}

const EFFECT_UPDATE_FREQ: u64 = 20; //ms //TODO: Make available in settings

impl EffectPlayerI {
    async fn start(mut self) {
        let mut sleep =
            rocket::tokio::time::interval(std::time::Duration::from_millis(EFFECT_UPDATE_FREQ));
        loop {
            select! {
                _ = sleep.tick() => {
                    self.update().await;
                }
                Ok(msg) = self.rx.recv() => {
                    if self.handle_action(msg).await {
                        break;
                    }
                }
            }
        }
    }

    async fn handle_action(&mut self, msg: EffectPlayerAction) -> bool {
        match msg {
            EffectPlayerAction::Stop => return true,
            EffectPlayerAction::Rebake => {
                self.bake_effects().await; //TODO: Make player not wait for baking to finish just queue and update when baking completed then also needs to run on seperate thread
            }
            EffectPlayerAction::EffectsChanged { id } => {
                self.bake_effect(id).await; //TODO: Same as above but then also needs to run on seperate thread
            }
            EffectPlayerAction::Toggle { id } => {
                if let Some(e) = self.effects.get_mut(&id) {
                    e.toggle()
                }
            }
        }

        false
    }

    async fn update(&mut self) {
        let now = chrono::Utc::now().naive_utc().time();
        let elapsed = now - self.time;
        self.time = now;

        let mut value_map = HashMap::new();

        for e in self.effects.values_mut() {
            if !e.running || e.max_time < Duration::milliseconds(2) {
                continue;
            }

            e.current_time += elapsed;
            if e.current_time > e.max_time {
                if e.looping {
                    while e.current_time > e.max_time {
                        e.current_time -= e.max_time;
                    }
                } else {
                    e.running = false;
                }
            }

            for f in &e.faders {
                let mut value = 0;
                for (d, v) in f.1.iter() {
                    if &e.current_time > d {
                        value = *v;
                    }
                }
                value_map.insert(*f.0, value);
            }
        }

        let mut universes = vec![];
        let mut channels = vec![];
        let mut values = vec![];

        for (k, v) in value_map {
            universes.push(k.universe);
            channels.push(k.address);
            values.push(v);
        }
        if !universes.is_empty() {
            self.runtime.set_values(universes, channels, values).await;
        }
    }

    async fn bake_effects(&mut self) {
        self.effects.clear();

        let p = self.project.lock().await;
        let effects = &p.effects;

        let patched_fixtures = get_patched_fixtures_clone(&p);

        send!(
            self.baking,
            BakingNotification::Started("Baking effects started!".to_string())
        );
        let time = Instant::now();
        let s: Vec<_> = effects
            .iter()
            .map(|e| (e.id, baking::bake(e, patched_fixtures.clone())))
            .collect();
        let l = s.len();
        send!(
            self.baking,
            BakingNotification::Misc("Baking effects running!".to_string())
        );

        for (id, ele) in s {
            let baked = ele.await;
            self.effects.insert(id, baked);
        }

        let duration = time.elapsed();
        send!(
            self.baking,
            BakingNotification::Finished(format!("Baking {} effects completed in {}", l, duration))
        );
        println!("Finished Baking!");
    }

    async fn bake_effect(&mut self, effect: uuid::Uuid) {
        let p = self.project.lock().await;
        let effects = &p.effects;
        let e = effects.iter().find(|e| e.id == effect);

        let patched_fixtures = get_patched_fixtures_clone(&p);

        send!(
            self.baking,
            BakingNotification::Started("Baking single effect!".to_string())
        );
        let time = Instant::now();
        if let Some(e) = e {
            let baked = baking::bake(e, patched_fixtures).await;
            self.effects.insert(e.id, baked);
        }

        let duration = time.elapsed();
        send!(
            self.baking,
            BakingNotification::Finished(format!("Baking single effect complete in {}", duration))
        );
        println!("Done");
    }
}

fn get_patched_fixtures_clone(p: &MutexGuard<ProjectI>) -> BakedFixtureData {
    use get_size::GetSize;
    let patched_fixtures: Vec<_> = p
        .universes
        .values()
        .flat_map(|u| u.get_fixtures().clone())
        .collect();
    println!(
        "Debug: Patched fixture clone size for baking {} bytes",
        patched_fixtures.get_heap_size()
    );
    patched_fixtures
}
