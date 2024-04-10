use std::collections::HashMap;
use std::time::Duration;

use schemars::JsonSchema;

use crate::patched::UniverseId;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default, JsonSchema)]
pub struct EndPointConfig {
    pub endpoints: HashMap<UniverseId, Vec<EPConfigItem>>,
}

#[derive(Debug, serde::Serialize, PartialEq, serde::Deserialize, Clone, JsonSchema)]
pub enum EPConfigItem {
    Logger,
    ArtNet,
    Sacn { universe: u16, speed: Speed },
    Usb { port: String, speed: Speed },
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone, Copy, JsonSchema)]
pub enum Speed {
    Slow,
    // 200ms
    Medium,
    // 100ms
    Fast, // 30ms
    SuperFast, // 5ms
}

impl Speed {
    pub fn get_duration(&self) -> Duration {
        match self {
            Speed::Slow => Duration::from_millis(200),
            Speed::Medium => Duration::from_millis(100),
            Speed::Fast => Duration::from_millis(30),
            Speed::SuperFast => Duration::from_millis(5),
        }
    }
}
