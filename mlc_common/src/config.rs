use std::collections::HashMap;

use get_size::GetSize;
use schemars::JsonSchema;

use crate::utils::{
    bounds::{One, Zero},
    BoundedValue,
};

pub type Value = Percentage;

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Clone,
    Copy,
    get_size::GetSize,
    JsonSchema,
)]
pub struct Percentage(BoundedValue<f64, Zero, One>);

impl Percentage {
    pub fn new(p: f64) -> Percentage {
        Percentage(BoundedValue::create(p))
    }

    pub fn from_dmx(dmx: usize, resolution: ValueResolution) -> Percentage {
        Percentage::new(dmx as f64 / (resolution.max() as f64))
    }

    pub fn to_dmx(&self, resolution: ValueResolution) -> u32 {
        (resolution.max() as f64 * *self.0) as u32
    }

    pub fn raw(&self) -> f64 {
        *self.0
    }

    pub fn zero() -> Self {
        Self::new(0.0)
    }
    pub fn full() -> Self {
        Self::new(1.0)
    }
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct FixtureType {
    pub name: String,
    pub short_name: String,
    pub categories: Vec<String>,
    pub fixture_key: String,
    pub modes: Vec<FixtureMode>,
    pub available_channels: HashMap<String, FixtureChannel>,
    pub matrix: Matrix,
    #[get_size(ignore)]
    pub id: uuid::Uuid,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
// 3D Array of Vec of group names
pub struct Matrix {
    pub mat: Vec<Vec<Vec<Vec<String>>>>,
    pub dimensions: [usize; 3],
}

impl FixtureType {
    pub fn get_modes(&self) -> &[FixtureMode] {
        &self.modes
    }

    pub fn get_available_channels(&self) -> &HashMap<String, FixtureChannel> {
        &self.available_channels
    }
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct FixtureChannel {
    pub default_value: Value,
    pub pixel_key: Option<String>,
    pub fine_channel_aliases: Vec<String>,
    pub capabilities: Vec<FixtureCapabilityCommon>,
}

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Clone,
    Copy,
    get_size::GetSize,
    JsonSchema,
)]
pub enum ValueResolution {
    U8,
    U16,
    U24,
}

impl ValueResolution {
    pub fn max(&self) -> u32 {
        let e = match self {
            ValueResolution::U8 => 8,
            ValueResolution::U16 => 16,
            ValueResolution::U24 => 24,
        };

        2_u32.pow(e) - 1
    }
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct Intensity {
    pub brightness_start: Brightness,
    pub brightness_end: Brightness,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub enum Brightness {
    Percentage(Percentage),
    Lumen(f32),
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct ColorIntensity {
    pub color: DmxColor,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct Rotation {
    pub speed_start: RotationSpeed,
    pub speed_end: RotationSpeed,
}

#[derive(
    Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, get_size::GetSize, JsonSchema,
)]
pub enum RotationSpeed {
    SlowCw,
    SlowCcw,
    FastCw,
    FastCcw,
    Stop,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct PanTilt {
    pub angle_start: PanTiltRotation,
    pub angle_end: PanTiltRotation,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub enum PanTiltRotation {
    Percentage(Percentage),
    Angle(u32),
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct PanTiltSpeed {
    speed_start: Speed,
    speed_end: Speed,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub enum Speed {
    Fast,
    Slow,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct FixtureCapabilityCommon {
    pub dmx_range: DmxRange,

    #[serde(flatten)]
    pub detail: FixtureCapability,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
#[serde(tag = "type")]
pub enum FixtureCapability {
    NoFunction,
    Maintenance,
    Intensity(Intensity),
    ColorIntensity(ColorIntensity),
    ColorPreset,
    ShutterStrobe,
    EffectSpeed,
    Rotation(Rotation),
    Pan(PanTilt),
    Tilt(PanTilt),
    PanTiltSpeed(PanTiltSpeed),
    Generic,
    Unimplemented,
}

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Clone,
    Copy,
    get_size::GetSize,
    JsonSchema,
)]
pub struct DmxRange {
    pub start: Percentage,
    pub end: Percentage,
}

impl DmxRange {
    pub fn full() -> Self {
        DmxRange {
            start: Percentage::zero(),
            end: Percentage::full(),
        }
    }
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
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
    Cyan,
    Magenta,
    Yellow,
    #[serde(alias = "Warm White")]
    WarmWhite,
    #[serde(alias = "Cold White")]
    ColdWhite,
    UV,
    Lime,
    Indigo,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct Manufacturer {
    name: String,
    website: String,
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize, JsonSchema,
)]
pub struct FixtureMode {
    pub name: String,
    pub short_name: String,
    pub channels: Vec<String>,
}
