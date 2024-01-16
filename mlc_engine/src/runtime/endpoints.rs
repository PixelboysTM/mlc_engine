use std::collections::HashMap;

use rocket::tokio::sync::broadcast::Receiver;

use crate::fixture::{UniverseAddress, UniverseId};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EndPointConfig {
    endpoints: HashMap<UniverseId, Vec<EPConfigItem>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum EPConfigItem {
    Logger,
}

pub trait Endpoint {
    fn register(&mut self, rx: Receiver<EndpointData>);
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
}

pub struct LoggerEndpoint;
impl Endpoint for LoggerEndpoint {
    fn register(&mut self, mut rx: Receiver<EndpointData>) {
        rocket::tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                println!("LOG: {}", serde_json::to_string(&msg).unwrap());
            }
        });
    }
}
