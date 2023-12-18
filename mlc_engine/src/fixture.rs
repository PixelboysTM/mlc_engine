mod ofl_parser;

pub use ofl_parser::parse_ofl_fixture;
use serde_with::formats::PreferMany;
use serde_with::serde_as;
use serde_with::OneOrMany;

use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureType {
    name: String,
    short_name: String,
    categories: Vec<String>,
    fixture_key: String,
    manufacturer_key: String,
    modes: Vec<FixtureMode>,
    available_channels: HashMap<String, FixtureChannel>,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureMode {
    name: String,
    short_name: String,
    channels: Vec<String>,
}

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureChannel {
    #[serde(default = "zero")]
    default_value: u8,
    #[serde(default = "all")]
    highlight_value: u8,
    #[serde(alias = "capability")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    capabilities: Vec<FixtureCapability>,
}

fn zero() -> u8 {
    0
}
fn all() -> u8 {
    255
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoFunction {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Maintenance {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Intensity {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorIntensity {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
    pub color: DmxColor,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
pub enum FixtureCapability {
    NoFunction(NoFunction),
    Maintenance(Maintenance),
    Intensity(Intensity),
    ColorIntensity(ColorIntensity),
    ColorPreset,
    ShutterStrobe,
    Effect(Effect),
    EffectSpeed,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DmxRange {
    start: u8,
    end: u8,
}

fn full_range() -> DmxRange {
    DmxRange { start: 0, end: 255 }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum DmxColor {
    Red,
    Green,
    Blue,
    White,
}
