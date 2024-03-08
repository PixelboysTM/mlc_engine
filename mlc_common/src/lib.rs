use chrono::{DateTime, Local};
use crate::config::{DmxRange, FixtureMode};

use crate::patched::{UniverseAddress, UniverseId};
use crate::universe::UNIVERSE_SIZE;

pub mod patched;
pub mod config;
pub mod universe;
pub mod fixture;
pub mod endpoints;

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

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct FixtureInfo {
    pub name: String,
    pub id: uuid::Uuid,
    pub modes: Vec<FixtureMode>,
}

pub trait ToFaderValue {
    fn to_fader_value(&self) -> u8;
    fn to_fader_value_range(&self, range: &DmxRange) -> u8;
    fn to_fader_value_range_fine(&self, range: &DmxRange) -> (u8, u8);
    fn to_fader_value_range_grain(&self, range: &DmxRange) -> (u8, u8, u8);
}

impl ToFaderValue for f32 {
    fn to_fader_value(&self) -> u8 {
        let v = self.min(1.0).max(0.0);
        (255.0 * v) as u8
    }

    fn to_fader_value_range(&self, range: &DmxRange) -> u8 {
        let v = self.min(1.0).max(0.0);
        (range.range(0, 255) as f32 * v) as u8 + range.start.to_value(0, 255) as u8
    }

    fn to_fader_value_range_fine(&self, range: &DmxRange) -> (u8, u8) {
        let v = self.min(1.0).max(0.0);
        let val = (range.range(0, 65535) as f32 * v) as u16 + range.start.to_value(0, 65535) as u16;
        ((val >> 8) as u8, val as u8)
    }

    fn to_fader_value_range_grain(&self, range: &DmxRange) -> (u8, u8, u8) {
        let v = self.min(1.0).max(0.0);
        let val =
            (range.range(0, 16777215) as f32 * v) as u32 + range.start.to_value(0, 16777215) as u32;
        ((val >> 16) as u8, (val >> 8) as u8, val as u8)
    }
}