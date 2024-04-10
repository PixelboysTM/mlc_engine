use std::collections::HashMap;

use rocket::tokio::sync::broadcast::{Receiver, Sender};

use mlc_common::endpoints::{EndPointConfig, EPConfigItem};
use mlc_common::patched::{UniverseAddress, UniverseId};
use mlc_common::universe::UNIVERSE_SIZE;
use crate::runtime::endpoints::usb::UsbEndpoint;

use self::{artnet::ArtNetEndpoint, sacn::SacnEndpoint};

mod artnet;
mod sacn;
mod usb;

macro_rules! register_default {
    ($type:ty, $rx:expr) => {
        <$type>::default().register($rx);
    };
}

pub trait CreateEndpoints {
    async fn create_endpoints(&self) -> HashMap<UniverseId, Vec<Sender<EndpointData>>>;
}

impl CreateEndpoints for EndPointConfig {
    async fn create_endpoints(&self) -> HashMap<UniverseId, Vec<Sender<EndpointData>>> {
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
                    EPConfigItem::Usb { speed, port } => UsbEndpoint {
                        port: port.clone(),
                        speed: *speed,
                    }.register(rx),
                }
                point.push(tx);
            }
            points.insert(*k, point);
        }

        points
    }
}

pub trait Endpoint: Default {
    fn register(self, rx: Receiver<EndpointData>);
}

#[allow(clippy::large_enum_variant)]
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

#[derive(Default)]
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
