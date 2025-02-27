use std::fmt::{Display, Formatter};
use std::str::FromStr;

use get_size::GetSize;
use schemars::JsonSchema;

use crate::config::DmxRange;
use crate::fixture::FaderAddress;
use crate::utils::bounds::{NegOne, One, Zero};
use crate::utils::BoundedValue;

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema, PartialEq,
)]
pub struct Dimmer {
    pub dimmer: FeatureTile,
}

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema, PartialEq,
)]
pub struct Rgb {
    pub red: FeatureTile,
    pub green: FeatureTile,
    pub blue: FeatureTile,
}

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema, PartialEq,
)]
pub struct Rotation {
    pub cw: FeatureTile,
    pub ccw: FeatureTile,
}

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema, PartialEq,
)]
pub struct PanTilt {
    pub pan: FeatureTile,
    pub tilt: FeatureTile,
}

// Indexes are offsets from the start_index of the Fixture
#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema, PartialEq,
)]
pub enum FixtureFeature {
    Dimmer(Dimmer),
    White(Dimmer),
    Amber(Dimmer),
    Rgb(Rgb),
    Rotation(Rotation),
    PanTilt(PanTilt),
}

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    PartialEq,
    get_size::GetSize,
    JsonSchema,
)]
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

impl FromStr for FixtureFeatureType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Dimmer" => Ok(FixtureFeatureType::Dimmer),
            "White" => Ok(FixtureFeatureType::White),
            "Rgb" => Ok(FixtureFeatureType::Rgb),
            "Rotation" => Ok(FixtureFeatureType::Rotation),
            "PanTilt" => Ok(FixtureFeatureType::PanTilt),
            "Amber" => Ok(FixtureFeatureType::Amber),
            s => Err(format!("Unknown FixtureFeatureType String: {s}")),
        }
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

pub trait HasFixtureFeature {
    fn has(&self, feature: &FixtureFeatureType) -> bool;
}

impl HasFixtureFeature for Vec<FixtureFeature> {
    fn has(&self, feature: &FixtureFeatureType) -> bool {
        for f in self {
            if f.name() == *feature {
                return true;
            }
        }

        false
    }
}

impl HasFixtureFeature for Vec<FixtureFeatureType> {
    fn has(&self, feature: &FixtureFeatureType) -> bool {
        self.contains(feature)
    }
}

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema, PartialEq,
)]
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum FeatureSetRequest {
    // 0.0 -> 1.0
    Dimmer {
        value: BoundedValue<f64, Zero, One>,
    },
    // (0.0, 0.0, 0.0) -> (1.0, 1.0, 1.0)
    Rgb {
        red: BoundedValue<f64, Zero, One>,
        green: BoundedValue<f64, Zero, One>,
        blue: BoundedValue<f64, Zero, One>,
    },
    // 0.0 -> 1.0
    White {
        value: BoundedValue<f64, Zero, One>,
    },
    // 0.0 -> 1.0
    Amber {
        value: BoundedValue<f64, Zero, One>,
    },
    // -1.0 -> 1.0  TODO: Needs an update in naming etc. is not clear what is meant (is it speed, value, ...)
    Rotation {
        value: BoundedValue<f64, NegOne, One>,
    },
    // (0.0, 0.0) -> (1.0, 1.0)
    PanTilt {
        pan: BoundedValue<f64, Zero, One>,
        tilt: BoundedValue<f64, Zero, One>,
    },
    GetAvailableFeatures,
}

/// The Offset of channelss from the start of the Fixture Fader = start_index + self
#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema, PartialEq,
)]
pub struct FeatureChannel(pub usize);
