use get_size::GetSize;
use schemars::JsonSchema;
use crate::patched::{UniverseAddress, UniverseId};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, get_size::GetSize, JsonSchema)]
pub struct FaderAddress {
    pub universe: UniverseId,
    pub address: UniverseAddress,
}