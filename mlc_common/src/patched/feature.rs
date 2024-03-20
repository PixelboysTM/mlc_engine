use std::fmt::{Display, Formatter};

use get_size::GetSize;
use schemars::JsonSchema;

use crate::config::DmxRange;
use crate::fixture::FaderAddress;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct Dimmer {
    pub dimmer: FeatureTile,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct Rgb {
    pub red: FeatureTile,
    pub green: FeatureTile,
    pub blue: FeatureTile,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct Rotation {
    pub cw: FeatureTile,
    pub ccw: FeatureTile,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct PanTilt {
    pub pan: FeatureTile,
    pub tilt: FeatureTile,
}

// Indexes are offsets from the start_index of the Fixture
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub enum FixtureFeature {
    Dimmer(Dimmer),
    White(Dimmer),
    Amber(Dimmer),
    Rgb(Rgb),
    Rotation(Rotation),
    PanTilt(PanTilt),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, get_size::GetSize)]
pub enum FixtureFeatureType {
    Dimmer,
    White,
    Rgb,
    Rotation,
    PanTilt,
    Amber,
}

impl Display for FixtureFeatureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FixtureFeatureType::Dimmer => "Dimmer",
            FixtureFeatureType::White => "White",
            FixtureFeatureType::Rgb => "Rgb",
            FixtureFeatureType::Rotation => "Rotation",
            FixtureFeatureType::PanTilt => "PanTilt",
            FixtureFeatureType::Amber => "Amber",
        })
    }
}

impl FixtureFeature {
    pub fn name(&self) -> FixtureFeatureType {
        match self {
            FixtureFeature::Dimmer(_) => FixtureFeatureType::Dimmer,
            FixtureFeature::White(_) => FixtureFeatureType::White,
            FixtureFeature::Rgb(_) => FixtureFeatureType::Rgb,
            FixtureFeature::Rotation(_) => FixtureFeatureType::Rotation,
            FixtureFeature::PanTilt(_) => FixtureFeatureType::PanTilt,
            FixtureFeature::Amber(_) => FixtureFeatureType::Amber,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub enum FeatureTile {
    Single {
        channel: FeatureChannel,
        fader: FaderAddress,
        range: DmxRange,
    },
    Double {
        channel: FeatureChannel,
        channel_fine: FeatureChannel,
        fader: FaderAddress,
        fader_fine: FaderAddress,
        range: DmxRange,
    },
    Tripple {
        channel: FeatureChannel,
        channel_fine: FeatureChannel,
        channel_grain: FeatureChannel,
        fader: FaderAddress,
        fader_fine: FaderAddress,
        fader_grain: FaderAddress,
        range: DmxRange,
    },
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum FeatureSetRequest {
    // 0.0 -> 1.0
    Dimmer { value: f32 },
    // (0.0, 0.0, 0.0) -> (1.0, 1.0, 1.0)
    Rgb { red: f32, green: f32, blue: f32 },
    // 0.0 -> 1.0
    White { value: f32 },
    // 0.0 -> 1.0
    Amber { value: f32 },
    // -1.0 -> 1.0  TODO: Needs an update in naming etc. is not clear what is meant (is it speed, value, ...)
    Rotation { value: f32 },
    // (0.0, 0.0) -> (1.0, 1.0)
    PanTilt { pan: f32, tilt: f32 },
    GetAvailableFeatures,
}


/// The Offset of channelss from the start of the Fixture Fader = start_index + self
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct FeatureChannel(pub usize);

