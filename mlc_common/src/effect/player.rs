use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::utils::{
    bounds::{One, Zero},
    BoundedValue,
};

use super::EffectId;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum EffectPlayerMsg {
    PlayingEffects { effects: Vec<EffectId> },
    EffectProgresses(Vec<(EffectId, BoundedValue<f32, Zero, One>)>),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum EffectPlayerRequest {
    Play { effect: EffectId },
    Stop { effect: EffectId },
}
