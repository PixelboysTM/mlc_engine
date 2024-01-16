mod artnet;

use std::collections::HashMap;

use rocket::tokio::sync::broadcast::{Receiver, Sender};

use crate::fixture::{UniverseAddress, UniverseId};

use self::artnet::ArtNetEndpoint;

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

impl EndPointConfig {
    pub async fn create_endpoints(&self) -> HashMap<UniverseId, Vec<Sender<EndpointData>>> {
        let mut points = HashMap::new();
        for (k, v) in &self.endpoints {
            let mut point = vec![];
            for items in v {
                let (tx, rx) = rocket::tokio::sync::broadcast::channel::<EndpointData>(500);
                match items {
                    EPConfigItem::Logger => {
                        LoggerEndpoint.register(rx);
                    }
                    EPConfigItem::ArtNet => {
                        let l = ArtNetEndpoint::default();
                        l.register(rx);
                    }
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
}

pub trait Endpoint {
    fn register(self, rx: Receiver<EndpointData>);
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EndpointData {
    Single {
        channel: UniverseAddress,
        value: u8,
    },
    Multiple {
        channels: Vec<UniverseAddress>,
        values: Vec<u8>,
    },
    Exit,
}

pub struct LoggerEndpoint;
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
