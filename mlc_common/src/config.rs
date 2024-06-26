use std::collections::HashMap;

use get_size::GetSize;
use schemars::JsonSchema;

pub type Value = DMXValue;

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
pub enum DMXValue {
    Percentage(Percentage),
    DMX {
        value: u32,
        resolution: ValueResolution,
    },
}

impl DMXValue {
    pub fn from_percentage(percent: Percentage) -> Self {
        Self::Percentage(percent)
    }

    pub fn from_dmx(value: u32, resolution: ValueResolution) -> Self {
        Self::DMX { value, resolution }
    }

    pub fn get_percent(&self) -> Percentage {
        match self {
            DMXValue::Percentage(p) => *p,
            DMXValue::DMX { value, resolution } => Percentage::dmx(*value as usize, *resolution),
        }
    }

    pub fn get_with_resolution(&self, dest_resolution: &ValueResolution) -> u32 {
        match self {
            DMXValue::Percentage(p) => (dest_resolution.max() as f32 * p.0) as u32,
            DMXValue::DMX { value, resolution } => {
                if resolution == dest_resolution {
                    *value
                } else {
                    let p = Percentage::dmx(*value as usize, *resolution);
                    (dest_resolution.max() as f32 * p.0) as u32
                }
            }
        }
    }
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
pub struct Percentage(f32);

impl Percentage {
    pub fn new(p: f32) -> Percentage {
        if p > 1.0 {
            println!("Capping percentage to 100%");
            Percentage(1.0)
        } else if p < 0.0 {
            println!("Capping percentage to 0%");
            Percentage(0.0)
        } else {
            Percentage(p)
        }
    }

    pub fn dmx(dmx: usize, resolution: ValueResolution) -> Percentage {
        Percentage::new(dmx as f32 / (resolution.max() as f32))
    }

    pub fn raw(&self) -> f32 {
        self.0
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
    #[get_size(ignore)]
    pub id: uuid::Uuid,
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
pub struct Maintenance {}

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
pub struct Effect {}

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
    pub start: Value,
    pub end: Value,
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
