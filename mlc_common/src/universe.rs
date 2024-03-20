use schemars::JsonSchema;

use crate::patched::{PatchedFixture, UniverseId};

pub const UNIVERSE_SIZE: usize = 512;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, JsonSchema)]
pub struct PatchedChannelIndex {
    pub fixture_index: usize,
    pub channel_index: usize,
}

#[serde_with::serde_as]
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, JsonSchema)]
pub struct FixtureUniverse {
    pub id: UniverseId,
    #[serde_as(as = "[_;UNIVERSE_SIZE]")]
    #[schemars(with = "[Option<PatchedChannelIndex>]")]
    pub channels: [Option<PatchedChannelIndex>; UNIVERSE_SIZE],
    pub fixtures: Vec<PatchedFixture>,
}

impl FixtureUniverse {
    const INIT: Option<PatchedChannelIndex> = None;

    pub fn empty(id: UniverseId) -> Self {
        FixtureUniverse {
            id,
            channels: [Self::INIT; UNIVERSE_SIZE],
            fixtures: vec![],
        }
    }
}



