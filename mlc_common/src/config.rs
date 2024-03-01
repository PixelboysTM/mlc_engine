use serde_with::formats::PreferMany;
use serde_with::OneOrMany;
use std::collections::HashMap;
use get_size::GetSize;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub type Value = u32;

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureType {
    name: String,
    categories: Vec<String>,
    fixture_key: String,
    manufacturer: Manufacturer,
    modes: Vec<FixtureMode>,
    available_channels: HashMap<String, FixtureChannel>,
    #[serde(default)]
    #[get_size(ignore)]
    pub id: uuid::Uuid,
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


#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureChannel {
    #[serde(default = "zero")]
    pub default_value: Value,

    #[serde(default)]
    pub dmx_value_resolution: ValueResolution,

    #[serde(default)]
    pub pixel_key: Option<String>,

    #[serde(default)]
    pub fine_channel_aliases: Vec<String>,
    #[serde(alias = "capability")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub capabilities: Vec<FixtureCapabilityCommon>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, Default, Copy, get_size::GetSize)]
pub enum ValueResolution {
    #[default]
    Implied,
    #[serde(alias = "8bit")]
    U8,
    #[serde(alias = "16bit")]
    U16,
    #[serde(alias = "24bit")]
    U24,
}

fn zero() -> Value {
    0
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct NoFunction {}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct Maintenance {}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct Intensity {}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct ColorIntensity {
    pub color: DmxColor,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    #[serde(alias = "speed")]
    pub speed_start: RotationSpeed,
    #[serde(default)]
    pub speed_end: RotationSpeed,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, get_size::GetSize, Default)]
pub enum RotationSpeed {
    #[serde(alias = "slow CW")]
    SlowCw,
    #[serde(alias = "slow CCW")]
    SlowCcw,
    #[serde(alias = "fast CW")]
    FastCw,
    #[serde(alias = "fast CCW")]
    FastCcw,
    #[serde(alias = "stop")]
    #[default]
    Stop,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct PanTilt {
    angle_start: u32,
    angle_end: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct PanTiltSpeed {
    speed_start: Speed,
    speed_end: Speed,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
pub enum Speed {
    #[serde(alias = "fast")]
    Fast,
    #[serde(alias = "slow")]
    Slow,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureCapabilityCommon {
    #[serde(default = "full_range")]
    pub dmx_range: DmxRange,

    #[serde(flatten)]
    pub detail: FixtureCapability,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
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
    Pan(PanTilt),
    Tilt(PanTilt),
    PanTiltSpeed(PanTiltSpeed),
    Generic,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct DmxRange {
    pub start: DmxRangeValue,
    pub end: DmxRangeValue,
}

#[derive(Debug, PartialEq, Clone, Copy, get_size::GetSize)]
pub enum DmxRangeValue {
    Value(Value),
    Percentage(f32),
}

impl DmxRange {
    pub fn range(&self, range_min: Value, range_max: Value) -> Value {
        self.end.to_value(range_min, range_max) - self.start.to_value(range_min, range_max)
    }
}

impl DmxRangeValue {
    pub fn to_value(&self, range_min: Value, range_max: Value) -> Value {
        match self {
            DmxRangeValue::Value(v) => *v.min(&range_max).max(&range_min),
            DmxRangeValue::Percentage(p) => {
                ((range_min as f32 + ((range_max as f32 - range_min as f32) * *p)) as Value)
                    .min(range_max)
                    .max(range_min)
            }
        }
    }
}

struct DmxRangeValueVisitor;

impl<'de> Visitor<'de> for DmxRangeValueVisitor {
    type Value = DmxRangeValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer float or string with pecentage")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(DmxRangeValue::Value(v))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(DmxRangeValue::Value(v as u32))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(DmxRangeValue::Value(v as u32))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(DmxRangeValue::Value(v as u32))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(DmxRangeValue::Value(v as u32))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(DmxRangeValue::Percentage(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(DmxRangeValue::Percentage(v as f32))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        if !v.ends_with("%") {
            return Err(E::custom("Not a percentage"));
        }

        let v = v
            .strip_suffix("%")
            .expect("Must be")
            .parse::<u8>()
            .map_err(|e| E::custom(e))?;

        Ok(DmxRangeValue::Percentage(v as f32 / 100.0))
    }
}

impl<'de> Deserialize<'de> for DmxRangeValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(DmxRangeValueVisitor)
    }
}

impl Serialize for DmxRangeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        match self {
            DmxRangeValue::Value(v) => serializer.serialize_u32(*v),
            DmxRangeValue::Percentage(f) => serializer.serialize_f32(*f),
        }
    }
}

fn full_range() -> DmxRange {
    DmxRange {
        start: DmxRangeValue::Percentage(0.0),
        end: DmxRangeValue::Percentage(1.0),
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
pub enum DmxColor {
    #[serde(alias = "#ff0000")]
    Red,
    #[serde(alias = "#00ff00")]
    Green,
    #[serde(alias = "#0000ff")]
    Blue,
    #[serde(alias = "#ffffff")]
    White,
    #[serde(alias = "#ffbf00")]
    Amber,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
pub struct Manufacturer {
    name: String,
    #[serde(default)]
    website: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
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