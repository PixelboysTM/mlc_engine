use chrono::Duration;
use serde_with::{DurationSecondsWithFrac, formats::Flexible, serde_as};

use crate::effect::{Effect, Track};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub enum EffectHandlerResponse {
    EffectCreated { name: String, id: uuid::Uuid },
    EffectUpdated { id: uuid::Uuid },
    EffectRunning { id: uuid::Uuid, running: bool },
    EffectList { effects: Vec<(String, uuid::Uuid)> },
    Effect { effect: Effect },
}

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
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