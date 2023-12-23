use serde_with::formats::PreferMany;
use serde_with::serde_as;
use serde_with::OneOrMany;

use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixtureType {
    name: String,
    categories: Vec<String>,
    fixture_key: String,
    manufacturer: Manufacturer,
    modes: Vec<FixtureMode>,
    available_channels: HashMap<String, FixtureChannel>,
    #[serde(default)]
    pub(super) id: uuid::Uuid,
}

#[allow(unused)]
impl FixtureType {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_categories(&self) -> &[String] {
        &self.categories
    }

    pub fn get_fixture_key(&self) -> &str {
        &self.fixture_key
    }

    pub fn get_manufacturer(&self) -> &Manufacturer {
        &self.manufacturer
    }

    pub fn get_modes(&self) -> &[FixtureMode] {
        &self.modes
    }

    pub fn get_available_channels(&self) -> &HashMap<String, FixtureChannel> {
        &self.available_channels
    }

    pub fn get_id(&self) -> &uuid::Uuid {
        &self.id
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixtureMode {
    name: String,
    short_name: String,
    channels: Vec<String>,
}

impl FixtureMode {
    pub fn get_channels(&self) -> &[String] {
        &self.channels
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FixtureChannel {
    #[serde(default = "zero")]
    default_value: u8,
    #[serde(alias = "capability")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    capabilities: Vec<FixtureCapability>,
}

fn zero() -> u8 {
    0
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoFunction {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Maintenance {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Intensity {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColorIntensity {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
    pub color: DmxColor,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum FixtureCapability {
    NoFunction(NoFunction),
    // Maintenance(Maintenance),
    Intensity(Intensity),
    ColorIntensity(ColorIntensity),
    // ColorPreset,
    ShutterStrobe,
    // Effect(Effect),
    // EffectSpeed,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DmxRange {
    start: u8,
    end: u8,
}

fn full_range() -> DmxRange {
    DmxRange { start: 0, end: 255 }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub enum DmxColor {
    #[serde(alias = "#ff0000")]
    Red,
    #[serde(alias = "#00ff00")]
    Green,
    #[serde(alias = "#0000ff")]
    Blue,
    #[serde(alias = "#ffffff")]
    White,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub struct Manufacturer {
    name: String,
    website: String,
}
