use crate::patched::UniverseId;
use schemars::JsonSchema;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default, JsonSchema)]
pub struct EndPointConfig {
    pub endpoints: HashMap<UniverseId, Vec<EPConfigItem>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, JsonSchema)]
pub enum EPConfigItem {
    Logger,
    ArtNet,
    Sacn { universe: u16, speed: Speed },
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, JsonSchema)]
pub enum Speed {
    Slow,
    // 200ms
    Medium,
    // 100ms
    Fast, // 30ms
}

impl Speed {
    pub fn get_duration(&self) -> Duration {
        match self {
            Speed::Slow => Duration::from_millis(200),
            Speed::Medium => Duration::from_millis(100),
            Speed::Fast => Duration::from_millis(30),
        }
    }
}
