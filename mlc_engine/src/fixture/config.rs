use regex::Regex;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Serialize;
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
    pub default_value: u8,
    #[serde(alias = "capability")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub capabilities: Vec<FixtureCapability>,
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
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,
    pub speed_start: RotationSpeed,
    pub speed_end: RotationSpeed,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RotationSpeed {
    SlowCw,
    SlowCcw,
    FastCw,
    FastCcw,
}

struct RotationSpeedVisitor;

impl<'de> Deserialize<'de> for RotationSpeed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(RotationSpeedVisitor)
    }
}

impl<'de> Visitor<'de> for RotationSpeedVisitor {
    type Value = RotationSpeed;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an string with a speed and a direction")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Some(c) = Regex::new(r"(?<speed>(slow)|(fast))\s+(?<dir>(CW)|(CCW))")
            .unwrap()
            .captures(v)
        {
            let speed = c.name("speed").unwrap().as_str();
            let dir = c.name("dir").unwrap().as_str();

            Ok(if speed == "slow" && dir == "CW" {
                RotationSpeed::SlowCw
            } else if speed == "slow" && dir == "CCW" {
                RotationSpeed::SlowCcw
            } else if speed == "fast" && dir == "CW" {
                RotationSpeed::FastCw
            } else if speed == "fast" && dir == "CCW" {
                RotationSpeed::FastCcw
            } else {
                return Err(E::custom("WHAT DE ACTUAL F"));
            })
        } else {
            Err(E::custom("Not matching"))
        }
    }
}

impl Serialize for RotationSpeed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            RotationSpeed::SlowCw => "slow CW",
            RotationSpeed::SlowCcw => "slow CCW",
            RotationSpeed::FastCw => "fast CW",
            RotationSpeed::FastCcw => "fast CCW",
        };
        serializer.serialize_str(s)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
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
    Rotation(Rotation),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct DmxRange {
    pub start: u8,
    pub end: u8,
}

impl DmxRange {
    pub fn range(&self) -> u8 {
        self.end - self.start
    }
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
