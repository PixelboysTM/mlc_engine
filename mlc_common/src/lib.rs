use chrono::{DateTime, Local};
use schemars::JsonSchema;
use serde::Serialize;
pub use uuid;

use crate::config::FixtureMode;
use crate::patched::{UniverseAddress, UniverseId};
use crate::universe::UNIVERSE_SIZE;

pub mod config;
pub mod endpoints;
pub mod fixture;
pub mod patched;
pub mod universe;

pub mod effect;

pub mod easing;
pub mod utils;

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
    RequireReload,
    None,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema, Clone)]
pub struct ProjectDefinition {
    pub name: String,
    pub last_edited: DateTime<Local>,
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub binary: bool,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, PartialEq)]
pub struct FixtureInfo {
    pub name: String,
    pub id: uuid::Uuid,
    pub modes: Vec<FixtureMode>,
}

pub fn to_save_file_name(file: &str) -> String {
    let mut new_name = String::new();
    for char in file.chars() {
        if char.is_whitespace() {
            if char == ' ' && !new_name.ends_with('_') {
                new_name += "_";
            }
            continue;
        }
        if char.is_ascii_alphanumeric() {
            new_name += &char.to_string();
            continue;
        }
        if ['_', '-'].contains(&char) {
            new_name += &char.to_string();
            continue;
        }
    }

    new_name = new_name.trim_matches(&[' ', '_']).to_string();

    new_name
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct CreateProjectData {
    pub name: String,
    pub binary: bool,
}

#[derive(serde::Deserialize, Serialize, JsonSchema)]
pub enum PatchResult {
    IdInvalid(String),
    ModeInvalid(String),
    Failed(String),
    Success(String),
}
