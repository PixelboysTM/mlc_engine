use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use rocket::{
    futures::{SinkExt, StreamExt},
    get, post,
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
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_ws::{Message, WebSocket};

use mlc_common::config::{DmxRange, Percentage, Value, ValueResolution};
use mlc_common::endpoints::EndPointConfig;
use mlc_common::patched::feature::{FeatureSetRequest, FixtureFeature};
use mlc_common::patched::{UniverseAddress, UniverseId};
use mlc_common::universe::UNIVERSE_SIZE;
use mlc_common::{FaderUpdateRequest, Info, RuntimeUpdate};

use crate::fixture::feature::ApplyFeature;
use crate::runtime::endpoints::CreateEndpoints;
use crate::{data_serving::ProjectGuard, module::Module, project::ProjectHandle, send};

use self::{effects::EffectModule, endpoints::EndpointData};

pub mod effects;
pub mod endpoints;

#[derive(Debug)]
struct RuntimeI {
    universe_values: HashMap<UniverseId, [u8; UNIVERSE_SIZE]>,
    //TODO: Only one Sender needed
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
    pub async fn adapt(&self, project: &ProjectHandle, clear: bool) {
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
                send!(
                    data.sender,
                    RuntimeUpdate::Universe {
                        universe,
                        values,
                        author: 0
                    }
                );
            }
        }

        {
            // Adapt Endpoints
            let c = project.get_endpoint_config().await;
            for v in data.end_points.values() {
                for vs in v {
                    send!(vs, EndpointData::Exit);
                }
            }
            sleep(Duration::from_millis(800)).await; // To allow port freeing
            let t = c.create_endpoints().await;
            data.end_points = t;
            for (id, v) in &data.end_points {
                if let Some(i) = data.universe_values.get(id) {
                    for vs in v {
                        send!(vs, EndpointData::Entire { values: *i });
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
        } else {
            println!("No Values");
        }
    }
    pub async fn set_values(
        &self,
        universes: Vec<UniverseId>,
        channels: Vec<UniverseAddress>,
        values: Vec<u8>,
    ) {
        let mut data = self.inner.lock().await;

        let mut u_u = vec![];
        let mut c_u = vec![];
        let mut v_u = vec![];

        for i in 0..universes.len() {
            let values_u = data.universe_values.get_mut(&universes[i]);
            if let Some(values_u) = values_u {
                let index: u16 = channels[i].into();
                values_u[index as usize] = values[i];
                u_u.push(universes[i]);
                c_u.push(channels[i]);
                v_u.push(values[i]);
                // self.update_endpoints(universe, channel, value, &data.end_points)
                //     .await;
            }
        }

        send!(
            data.sender,
            RuntimeUpdate::ValuesUpdated {
                universes: u_u.clone(),
                channel_indexes: c_u.iter().map(|i| (*i).into()).collect(),
                values: v_u.clone()
            }
        );
        self.update_endpoints_batch(u_u, c_u, v_u, &data.end_points)
            .await;
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

    async fn update_endpoints_batch(
        &self,
        verse_ids: Vec<UniverseId>,
        indexs: Vec<UniverseAddress>,
        values: Vec<u8>,
        endpoints: &HashMap<UniverseId, Vec<Sender<EndpointData>>>,
    ) {
        let mut map: HashMap<UniverseId, Vec<(UniverseAddress, u8)>> = HashMap::new();
        for (i, verse_id) in verse_ids.into_iter().enumerate() {
            if let Entry::Vacant(_) = map.entry(verse_id) {
                map.insert(verse_id, vec![(indexs[i], values[i])]);
            } else {
                map.get_mut(&verse_id)
                    .expect("Checked")
                    .push((indexs[i], values[i]));
            }
        }
        for verse_id in map.keys() {
            let point = endpoints.get(verse_id);
            if let Some(point) = point {
                let cs: Vec<_> = map
                    .get(verse_id)
                    .expect("Checked")
                    .iter()
                    .map(|i| i.0)
                    .collect();
                let vs: Vec<_> = map
                    .get(verse_id)
                    .expect("Checked")
                    .iter()
                    .map(|i| i.1)
                    .collect();
                for v in point {
                    send!(
                        v,
                        EndpointData::Multiple {
                            channels: cs.clone(),
                            values: vs.clone()
                        }
                    );
                }
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
        data.universe_values.get(universe).copied()
    }
}

pub struct RuntimeModule;

impl Module for RuntimeModule {
    fn setup(
        &self,
        app: rocket::Rocket<rocket::Build>,
        spec: &mut OpenApi,
    ) -> rocket::Rocket<rocket::Build> {
        let (tx, rx) = broadcast::channel::<RuntimeUpdate>(512);

        let (routes, s) = openapi_get_routes_spec![
            get_value_updates,
            set_value,
            get_endpoint_config,
            set_endpoint_config,
            set_feature
        ];
        merge_specs(spec, &"/runtime".to_string(), &s).expect("Failed merging OpenApi");

        let app = app
            .manage(rx)
            .manage(RuntimeData::new(tx))
            .mount("/runtime", routes);
        EffectModule.setup(app, spec)
    }
}

/// # Fader values 'get'
/// Upgrades to a WebSocket which receives updates about changing Dmx Fader Values in universes
///
/// Send a UniverseId to get an exclusive update of that specified Universe.
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Runtime")]
#[get("/fader-values/get")]
async fn get_value_updates(
    runtime: &State<RuntimeData>,
    ws: WebSocket,
    mut shutdown: Shutdown,
    _g: ProjectGuard,
) -> rocket_ws::Channel<'_> {
    let mut rx = runtime.subscribe().await;
    let init = runtime.initial_states().await;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            for key in init.keys() {
                stream.send(rocket_ws::Message::text(serde_json::to_string(&RuntimeUpdate::Universe { universe: *key, values: *init.get(key).expect("In for each"), author: 0 }).unwrap())).await.unwrap();
            }

            loop {
                select! {
                    Ok(msg) = rx.recv() => {
                        let _ = stream.send(rocket_ws::Message::text(serde_json::to_string(&msg).unwrap())).await;
                    },
                    Some(msg) = stream.next() => {
                    if let Ok(msg) = msg {
                            if let Some(req) = decode_msg(&msg) {
                                let data = runtime.get_universe_values(&req).await;
                                if let Some(data) = data {
                                    stream.send(rocket_ws::Message::text(serde_json::to_string(&RuntimeUpdate::Universe { universe: req, values: data, author: 0 }).unwrap())).await.unwrap();
                                }
                            }
                    }
                },
                    _ = &mut shutdown => {
                        break;
                    }
                }
            }

            Ok(())
        })
    })
}

fn decode_msg<'a, T: serde::Deserialize<'a>>(msg: &'a Message) -> Option<T> {
    if let Ok(json) = msg.to_text() {
        let s = serde_json::from_str(json);
        if let Ok(val) = s {
            return Some(val);
        }

        let err = s.err().unwrap();
        eprintln!("Error Decoding msg: {:?}", err);
    }

    None
}

/// # Fader values 'set'
/// Send [`mlc_common::FaderUpdateRequest`] to set one or multiple Dmx Channels directly
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Runtime")]
#[get("/fader-values/set")]
async fn set_value(
    runtime: &State<RuntimeData>,
    ws: WebSocket,
    mut shutdown: Shutdown,
    _g: ProjectGuard,
) -> rocket_ws::Channel<'_> {
    let rd = runtime;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    Some(msg) = stream.next() => {
                    if let Ok(msg) = msg {
                            if let Some(req) = decode_msg::<FaderUpdateRequest>(&msg) {
                                rd.set_value(req.universe, req.channel, req.value).await;
                            }
                    }
                },
                    _ = &mut shutdown => {
                        break;
                    },
                }
            }

            Ok(())
        })
    })
}

/// # Endpoints get
/// Returns the current [EndpointConfig][`mlc_common::endpoints::EndPointConfig`] of the project
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Runtime")]
#[get("/endpoints/get")]
async fn get_endpoint_config(
    project: &State<ProjectHandle>,
    _g: ProjectGuard,
) -> Json<EndPointConfig> {
    let config = project.get_endpoint_config().await;
    Json(config)
}

/// # Endpoints set
/// Updates the current endpoint config and reloads all associated services
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Runtime")]
#[post("/endpoints/set", data = "<data>")]
async fn set_endpoint_config(
    project: &State<ProjectHandle>,
    data: Json<EndPointConfig>,
    runtime: &State<RuntimeData>,
    tx: &State<Sender<Info>>,
    _g: ProjectGuard,
) -> Json<&'static str> {
    project.set_endpoint_config(data.0).await;
    runtime.adapt(project, false).await;
    send!(tx, Info::EndpointConfigChanged);
    Json("ok")
}

/// # Set Feature
/// Opens a WebSocket to a specific patched fixture. To manually control its features.
///
/// See [FeatureSetRequest][`mlc_common::patched::feature::FeatureSetRequest`]
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Runtime")]
#[get("/feature/<fix_id>")]
async fn set_feature<'a>(
    ws: WebSocket,
    mut shutdown: Shutdown,
    fix_id: &'a str,
    runtime: &'a State<RuntimeData>,
    project: &'a State<ProjectHandle>,
    _g: ProjectGuard,
) -> rocket_ws::Channel<'a> {
    let id = uuid::Uuid::from_str(fix_id);

    async fn get_features(id: uuid::Uuid, project: &ProjectHandle) -> Option<Vec<FixtureFeature>> {
        let universes = project.get_universes().await;
        for universe in universes {
            let u = project.get_universe(&universe).await.expect("Queried");
            let fs = &u.fixtures;
            for f in fs {
                if f.id == id {
                    return Some(f.features.clone());
                }
            }
        }

        None
    }

    ws.channel(move |mut stream| {
        Box::pin(async move {
            if let Ok(id) = id {
                if let Some(fs) = get_features(id, project).await {
                    let r = (runtime.inner()).clone();
                    loop {
                        select! {
                            pot_msg = stream.next() => {
                                if let Some(msg) = pot_msg {
                                    if let Ok(msg) = msg {
                                        if msg.is_close() {
                                            break;
                                        }

                                        if let Some(fsr) = decode_msg(&msg){
                                            if let FeatureSetRequest::GetAvailableFeatures = fsr {
                                                stream.send(rocket_ws::Message::text(serde_json::to_string(&fs.iter().map(|s| s.name()).collect::<Vec<_>>()).unwrap())).await.unwrap();
                                            } else {
                                                fs.apply(fsr, &r).await;
                                            }
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                            _ = &mut shutdown => {
                                break;
                            }
                        }
                    }
                }
            }
            Ok(())
        })
    })
}

pub trait ToFaderValue {
    fn to_fader_value_range(&self, range: &DmxRange) -> u8;
    fn to_fader_value_range_fine(&self, range: &DmxRange) -> (u8, u8);
    fn to_fader_value_range_grain(&self, range: &DmxRange) -> (u8, u8, u8);
}

impl ToFaderValue for f64 {
    fn to_fader_value_range(&self, range: &DmxRange) -> u8 {
        let v = self.min(1.0).max(0.0);
        let val = lerp(range.start, range.end, Percentage::new(v));
        val.to_dmx(ValueResolution::U8) as u8
    }

    fn to_fader_value_range_fine(&self, range: &DmxRange) -> (u8, u8) {
        let v = self.min(1.0).max(0.0);
        let val = lerp(range.start, range.end, Percentage::new(v)).to_dmx(ValueResolution::U16);
        ((val >> 8) as u8, val as u8)
    }

    fn to_fader_value_range_grain(&self, range: &DmxRange) -> (u8, u8, u8) {
        let v = self.min(1.0).max(0.0);
        let val = lerp(range.start, range.end, Percentage::new(v)).to_dmx(ValueResolution::U24);
        ((val >> 16) as u8, (val >> 8) as u8, val as u8)
    }
}

fn lerp(v0: Percentage, v1: Percentage, t: Percentage) -> Value {
    Percentage::new(v0.raw() + t.raw() * (v1.raw() - v0.raw()))
}
