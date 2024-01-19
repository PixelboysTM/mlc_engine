pub mod endpoints;

use std::{collections::HashMap, sync::Arc, time::Duration};

use rocket::{
    futures::{SinkExt, StreamExt},
    get, post, routes,
    serde::json::Json,
    tokio::{
        select,
        sync::{
            broadcast::{self, Receiver, Sender},
            Mutex,
        },
        time::sleep,
    },
    Shutdown, State,
};
use rocket_ws::WebSocket;

use crate::{
    data_serving::Info,
    fixture::{UniverseAddress, UniverseId, UNIVERSE_SIZE},
    module::Module,
    project::Project,
    send,
};

use self::endpoints::{EndPointConfig, EndpointData};

#[derive(Debug)]
struct RuntimeI {
    universe_values: HashMap<UniverseId, [u8; UNIVERSE_SIZE]>,
    end_points: HashMap<UniverseId, Vec<Sender<EndpointData>>>,
    sender: Sender<RuntimeUpdate>,
}

#[derive(Debug, Clone)]
pub struct RuntimeData {
    inner: Arc<Mutex<RuntimeI>>,
}

impl RuntimeData {
    fn new(sender: Sender<RuntimeUpdate>) -> RuntimeData {
        RuntimeData {
            inner: Arc::new(Mutex::new(RuntimeI {
                universe_values: HashMap::new(),
                end_points: HashMap::new(),
                sender,
            })),
        }
    }
    pub async fn adapt(&self, project: &Project, clear: bool) {
        let mut data = self.inner.lock().await;

        {
            // Adapt Universes
            let verses = data.universe_values.clone();
            data.universe_values.clear();
            for universe in project.get_universes().await {
                let values = if !clear && verses.contains_key(&universe) {
                    *verses.get(&universe).expect("Testet")
                } else {
                    [0; UNIVERSE_SIZE]
                };
                data.universe_values.insert(universe, values);
                send!(data.sender, RuntimeUpdate::Universe { universe, values });
            }
        }

        {
            // Adapt Endpoints
            let c = project.get_endpoint_config().await;
            for (_, v) in &data.end_points {
                for vs in v {
                    send!(vs, EndpointData::Exit);
                }
            }
            sleep(Duration::from_millis(500)).await;
            let t = c.create_endpoints().await;
            data.end_points = t;
            for (id, v) in &data.end_points {
                if let Some(i) = data.universe_values.get(id) {
                    for vs in v {
                        send!(vs, EndpointData::Entire { values: i.clone() });
                    }
                }
            }
        }
    }

    pub async fn set_value(&self, universe: UniverseId, channel: UniverseAddress, value: u8) {
        let mut data = self.inner.lock().await;

        let values = data.universe_values.get_mut(&universe);
        if let Some(values) = values {
            let index: u16 = channel.into();
            values[index as usize] = value;
            send!(
                data.sender,
                RuntimeUpdate::ValueUpdated {
                    universe,
                    channel_index: index as usize,
                    value,
                }
            );
            self.update_endpoints(universe, channel, value, &data.end_points)
                .await;
        }
    }

    async fn update_endpoints(
        &self,
        verse_id: UniverseId,
        index: UniverseAddress,
        value: u8,
        endpoints: &HashMap<UniverseId, Vec<Sender<EndpointData>>>,
    ) {
        let point = endpoints.get(&verse_id);
        if let Some(point) = point {
            for v in point {
                send!(
                    v,
                    EndpointData::Single {
                        channel: index,
                        value
                    }
                );
            }
        }
    }

    pub async fn subscribe(&self) -> Receiver<RuntimeUpdate> {
        let data = self.inner.lock().await;

        data.sender.subscribe()
    }

    pub async fn initial_states(&self) -> HashMap<UniverseId, [u8; UNIVERSE_SIZE]> {
        let data = self.inner.lock().await;
        data.universe_values.clone()
    }
    pub async fn get_universe_values(&self, universe: &UniverseId) -> Option<[u8; UNIVERSE_SIZE]> {
        let data = self.inner.lock().await;
        data.universe_values.get(universe).map(|f| f.clone())
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RuntimeUpdate {
    ValueUpdated {
        universe: UniverseId,
        channel_index: usize,
        value: u8,
    },
    Universe {
        universe: UniverseId,
        #[serde_as(as = "[_;UNIVERSE_SIZE]")]
        values: [u8; UNIVERSE_SIZE],
    },
}

pub struct RuntimeModule;

impl Module for RuntimeModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        let (tx, rx) = broadcast::channel::<RuntimeUpdate>(512);

        app.manage(rx).manage(RuntimeData::new(tx)).mount(
            "/runtime",
            routes![
                get_value_updates,
                set_value,
                get_endpoint_config,
                set_endpoint_config
            ],
        )
    }
}

#[get("/fader-values/get")]
async fn get_value_updates(
    runtime: &State<RuntimeData>,
    ws: WebSocket,
    mut shutdown: Shutdown,
) -> rocket_ws::Channel<'_> {
    let mut rx = runtime.subscribe().await;
    let init = runtime.initial_states().await;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            for key in init.keys() {
                stream.send(rocket_ws::Message::text(serde_json::to_string(&RuntimeUpdate::Universe { universe: *key, values: init.get(key).expect("In for each").clone() }).unwrap())).await.unwrap();
            }

            loop {
                select! {
                    Ok(msg) = rx.recv() => {
                        let _ = stream.send(rocket_ws::Message::text(serde_json::to_string(&msg).unwrap())).await;
                    },
                    Some(msg) = stream.next() => {
                    if let Ok(msg) = msg {
                        let req: UniverseId =
                            serde_json::from_str(msg.to_text().unwrap()).unwrap();
                        let data = runtime.get_universe_values(&req).await;
                        if let Some(data) = data {
                            stream.send(rocket_ws::Message::text(serde_json::to_string(&RuntimeUpdate::Universe { universe: req, values: data }).unwrap())).await.unwrap();
                        }
                    }
                },
                    _ = &mut shutdown => {
                        break;
                    }
                };
            }

            Ok(())
        })
    })
}

#[derive(serde::Deserialize)]
struct FaderUpdateRequest {
    universe: UniverseId,
    channel: UniverseAddress,
    value: u8,
}

#[get("/fader-values/set")]
async fn set_value(
    runtime: &State<RuntimeData>,
    ws: WebSocket,
    mut shutdown: Shutdown,
) -> rocket_ws::Channel<'_> {
    let rd = runtime.clone();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                // if let Some(msg) = stream.next().await {
                //     if let Ok(msg) = msg {
                //         let req: FaderUpdateRequest =
                //             serde_json::from_str(msg.to_text().unwrap()).unwrap();
                //         rd.set_value(req.universe, req.channel, req.value).await;
                //     }
                // }
                select! {
                    Some(msg) = stream.next() => {
                    if let Ok(msg) = msg {
                        let req: FaderUpdateRequest =
                            serde_json::from_str(msg.to_text().unwrap()).unwrap();
                        rd.set_value(req.universe, req.channel, req.value).await;
                    }
                },
                    _ = &mut shutdown => {
                        break;
                    },
                };
            }

            Ok(())
        })
    })
}

#[get("/endpoints/get")]
async fn get_endpoint_config(project: &State<Project>) -> Json<EndPointConfig> {
    let config = project.get_endpoint_config().await;
    Json(config)
}

#[post("/endpoints/set", data = "<data>")]
async fn set_endpoint_config(
    project: &State<Project>,
    data: Json<EndPointConfig>,
    runtime: &State<RuntimeData>,
    tx: &State<Sender<Info>>,
) {
    project.set_endpoint_config(data.0).await;
    runtime.adapt(project, false).await;
    send!(tx, Info::EndpointConfigChanged);
}
