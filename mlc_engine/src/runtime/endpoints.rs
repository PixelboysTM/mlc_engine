mod artnet;
mod sacn;

use std::collections::HashMap;

use rocket::tokio::sync::broadcast::{Receiver, Sender};

use crate::fixture::{UniverseAddress, UniverseId, UNIVERSE_SIZE};

use self::{
    artnet::ArtNetEndpoint,
    sacn::{SacnEndpoint, Speed},
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct EndPointConfig {
    endpoints: HashMap<UniverseId, Vec<EPConfigItem>>,
}

impl Default for EndPointConfig {
    fn default() -> Self {
        Self {
            endpoints: Default::default(),
        }
    }
}

macro_rules! register_default {
    ($type:ty, $rx:expr) => {
        <$type>::default().register($rx);
    };
}

impl EndPointConfig {
    pub async fn create_endpoints(&self) -> HashMap<UniverseId, Vec<Sender<EndpointData>>> {
        let mut points = HashMap::new();
        for (k, v) in &self.endpoints {
            let mut point = vec![];
            for items in v {
                let (tx, rx) = rocket::tokio::sync::broadcast::channel::<EndpointData>(500);
                match items {
                    EPConfigItem::Logger => {
                        register_default!(LoggerEndpoint, rx);
                    }
                    EPConfigItem::ArtNet => {
                        register_default!(ArtNetEndpoint, rx);
                    }
                    EPConfigItem::Sacn { universe, speed } => SacnEndpoint {
                        universe: *universe,
                        speed: speed.clone(),
                        ..Default::default()
                    }
                    .register(rx),
                }
                point.push(tx);
            }
            points.insert(*k, point);
        }

        points
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum EPConfigItem {
    Logger,
    ArtNet,
    Sacn { universe: u16, speed: Speed },
}

pub trait Endpoint: Default {
    fn register(self, rx: Receiver<EndpointData>);
}

#[derive(Debug, Clone)]
pub enum EndpointData {
    Single {
        channel: UniverseAddress,
        value: u8,
    },
    Multiple {
        channels: Vec<UniverseAddress>,
        values: Vec<u8>,
    },
    Entire {
        values: [u8; UNIVERSE_SIZE],
    },
    Exit,
}

pub struct LoggerEndpoint;
impl Default for LoggerEndpoint {
    fn default() -> Self {
        Self {}
    }
}
impl Endpoint for LoggerEndpoint {
    fn register(self, mut rx: Receiver<EndpointData>) {
        rocket::tokio::spawn(async move {
            let id = uuid::Uuid::new_v4();
            while let Ok(msg) = rx.recv().await {
                match msg {
                    EndpointData::Exit => {
                        println!("[DBG]: {:?} EXIT", &id);
                        break;
                    }
                    s => println!("[DBG]: {:?} {:?}", &id, s),
                }
            }
        });
    }
}
