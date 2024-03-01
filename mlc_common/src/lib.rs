use chrono::{DateTime, Local};

use crate::patched::{UniverseAddress, UniverseId};
use crate::universe::UNIVERSE_SIZE;

pub mod patched;
pub mod universe;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Copy)]
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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ProjectDefinition {
    pub name: String,
    #[serde(default)]
    pub file_name: String,
    pub last_edited: DateTime<Local>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Settings {
    pub save_on_quit: bool,
}

impl Settings {
    pub fn save_on_quit(&self) -> bool {
        self.save_on_quit
    }
}

#[allow(clippy::large_enum_variant)]
#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
        #[serde_as(as = "[_;UNIVERSE_SIZE]")]
        values: [u8; UNIVERSE_SIZE],
        author: usize,
    },
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FaderUpdateRequest {
    pub universe: UniverseId,
    pub channel: UniverseAddress,
    pub value: u8,
}
