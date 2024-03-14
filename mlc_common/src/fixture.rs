use crate::patched::{UniverseAddress, UniverseId};
use get_size::GetSize;
use schemars::JsonSchema;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    get_size::GetSize,
    JsonSchema,
)]
pub struct FaderAddress {
    pub universe: UniverseId,
    pub address: UniverseAddress,
}
