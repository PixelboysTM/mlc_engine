use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::EffectId;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum EffectPlayerMsg {
    PlayingEffects { effects: Vec<EffectId> },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum EffectPlayerRequest {
    Play { effect: EffectId },
    Stop { effect: EffectId },
}
