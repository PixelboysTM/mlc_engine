use chrono::{DateTime, Local};
use schemars::JsonSchema;

use crate::config::FixtureMode;
use crate::patched::{UniverseAddress, UniverseId};
use crate::universe::UNIVERSE_SIZE;

pub mod config;
pub mod endpoints;
pub mod fixture;
pub mod patched;
pub mod universe;


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Copy, JsonSchema)]
pub enum Info {
    FixtureTypesUpdated,
    ProjectSaved,
    ProjectLoaded,
    SystemShutdown,
    UniversePatchChanged(UniverseId),
    UniversesUpdated,
    EndpointConfigChanged,
    EffectListChanged,
    None,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ProjectDefinition {
    pub name: String,
    #[serde(default)]
    pub file_name: String,
    pub last_edited: DateTime<Local>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, JsonSchema)]
pub struct ProjectSettings {
    pub save_on_quit: bool,
}

#[allow(clippy::large_enum_variant)]
#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum RuntimeUpdate {
    ValueUpdated {
        universe: UniverseId,
        channel_index: usize,
        value: u8,
    },
    ValuesUpdated {
        universes: Vec<UniverseId>,
        channel_indexes: Vec<usize>,
        values: Vec<u8>,
    },
    Universe {
        universe: UniverseId,
        #[schemars(skip)]
        #[serde_as(as = "[_;UNIVERSE_SIZE]")]
        values: [u8; UNIVERSE_SIZE],
        author: usize,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct FaderUpdateRequest {
    pub universe: UniverseId,
    pub channel: UniverseAddress,
    pub value: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct FixtureInfo {
    pub name: String,
    pub id: uuid::Uuid,
    pub modes: Vec<FixtureMode>,
}
