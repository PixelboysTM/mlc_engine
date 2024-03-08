use std::collections::HashMap;
use std::time::Duration;
use crate::patched::UniverseId;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct EndPointConfig {
    pub endpoints: HashMap<UniverseId, Vec<EPConfigItem>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum EPConfigItem {
    Logger,
    ArtNet,
    Sacn { universe: u16, speed: Speed },
}


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Speed {
    Slow,
    // 200ms
    Medium,
    // 100ms
    Fast,   // 30ms
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