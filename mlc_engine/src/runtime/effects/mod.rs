use std::collections::HashMap;

use chrono::Duration;
use rocket::fairing::AdHoc;
use rocket::futures::{SinkExt, StreamExt};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{self, Receiver, Sender};
use rocket::{get, routes, Shutdown, State};
use rocket_ws::stream::DuplexStream;
use rocket_ws::WebSocket;
use serde_with::serde_as;
use serde_with::{formats::Flexible, DurationSecondsWithFrac};

use crate::data_serving::{Info, ProjectGuard};
use crate::project::Project;
use crate::{fixture::FaderAddress, module::Module, send};

use super::{decode_msg, RuntimeData};

pub struct EffectModule;

impl Module for EffectModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        let tx = startup_effect_player(
            app.state::<RuntimeData>().unwrap().clone(),
            app.state::<Project>().unwrap().clone(),
        );

        app.manage(tx)
            .attach(AdHoc::on_shutdown("Shutdown EffectPlayer", |a| {
                Box::pin(async move {
                    let _ = a
                        .state::<Sender<EffectPlayerAction>>()
                        .unwrap()
                        .send(EffectPlayerAction::Stop);
                })
            }))
            .mount("/effects", routes![get_effect_handler, get_effect_list])
    }
}

#[get("/get")]
async fn get_effect_list(
    project: &State<Project>,
    _g: ProjectGuard,
) -> Json<Vec<(String, uuid::Uuid)>> {
    let p = project.lock().await;
    Json(
        p.effects
            .iter()
            .map(|e| (e.name.to_string(), e.id.clone()))
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
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct FaderTrack {
    address: FaderAddress,
    values: Vec<FaderKey>,
}

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct FaderKey {
    value: u8,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    start_time: Duration,
}

#[derive(Debug, serde::Serialize)]
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
                            let req: EffectHandlerRequest = decode_msg(&msg).expect("Must be");
                            let send_info = matches!(req, EffectHandlerRequest::Create {..});

                            handle_msg(&mut stream, req, tx, project).await;
                            if send_info {
                                send!(info, Info::EffectListChanged);
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
                .map(|e| (e.name.clone(), e.id.clone()))
                .collect();
            let _ = stream
                .send(make_msg(&EffectHandlerResponse::EffectList { effects }))
                .await;
        }
    }
}

fn validate_effect_name(name: String) -> String {
    let parts = name.split("/");
    parts.map(|p| p.trim()).fold("".to_string(), |a, p| {
        if a.len() == 0 {
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
}

#[derive(Debug, Clone)]
pub enum EffectPlayerAction {
    Stop,
    EffectsChanged { id: uuid::Uuid },
    Rebake,
    Toggle { id: uuid::Uuid },
}

#[derive(Debug)]
pub struct BakedEffect {
    faders: HashMap<FaderAddress, Vec<(Duration, u8)>>,
    current_time: Duration,
    max_time: Duration,
    running: bool,
    looping: bool,
}

impl BakedEffect {
    fn toggle(&mut self) {
        if self.running {
            self.running = false;
        } else {
            self.current_time = Duration::milliseconds(0);
            self.running = true;
        }
    }
}

fn startup_effect_player(runtime: RuntimeData, project: Project) -> Sender<EffectPlayerAction> {
    let (tx, rx) = broadcast::channel(500);

    rocket::tokio::spawn(async move {
        let effect_player = EffectPlayerI {
            effects: HashMap::new(),
            project,
            runtime,
            rx,
            time: chrono::Utc::now().naive_utc().time(),
        };
        effect_player.start().await;
    });

    tx
}

const EFFECT_UPDATE_FREQ: u64 = 20; //TODO: Make available in settings

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
                self.bake_effects().await;
            }
            EffectPlayerAction::EffectsChanged { id } => {
                self.bake_effect(id).await;
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

        for (_, e) in &mut self.effects {
            if !e.running || e.max_time < Duration::milliseconds(2) {
                continue;
            }

            e.current_time = e.current_time + elapsed;
            if e.current_time > e.max_time {
                if e.looping {
                    while e.current_time > e.max_time {
                        e.current_time = e.current_time - e.max_time;
                    }
                } else {
                    e.running = false;
                }
            }

            for f in &e.faders {
                let mut value = 0;
                for (i, (d, v)) in f.1.iter().enumerate() {
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

        for ele in effects {
            let baked = bake(ele).await;
            self.effects.insert(ele.id, baked);
        }

        println!("Finished Baking!");
    }

    async fn bake_effect(&mut self, effect: uuid::Uuid) {
        let p = self.project.lock().await;
        let effects = &p.effects;
        let e = effects.iter().find(|e| e.id == effect);
        if let Some(e) = e {
            let baked = bake(e).await;
            self.effects.insert(e.id, baked);
        }
    }
}

async fn bake(effect: &Effect) -> BakedEffect {
    let mut faders = HashMap::new();

    for track in &effect.tracks {
        match track {
            Track::FaderTrack(cue) => {
                faders.insert(cue.address, bake_fader_cue(cue, &effect.duration))
            }
        };
    }

    BakedEffect {
        faders,
        max_time: effect.duration,
        current_time: Duration::milliseconds(0),
        running: false,
        looping: effect.looping,
    }
}

fn bake_fader_cue(fader_cue: &FaderTrack, max_time: &Duration) -> Vec<(Duration, u8)> {
    let mut vals: Vec<_> = fader_cue
        .values
        .iter()
        .filter(|k| &k.start_time <= max_time && k.start_time >= Duration::milliseconds(0))
        .map(|f| (f.start_time, f.value))
        .collect();
    vals.sort_by_key(|k| k.0);

    vals
}
