use ::serde_with::{DurationSecondsWithFrac, formats::Flexible, formats::PreferOne, OneOrMany};
use chrono::Duration;
use serde_with::serde_as;
use crate::effect::D2RotationKey;

use crate::effect::track_key::{D3PercentageKey, PercentageKey, RotationKey};
use crate::patched::feature::FixtureFeatureType;
use crate::patched::FixtureId;

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct FeatureTrack {
    #[serde(alias = "fixture")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub fixtures: Vec<FixtureId>,
    pub feature: FixtureFeatureType,
    pub detail: FeatureTrackDetail,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    pub resolution: Duration,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub enum FeatureTrackDetail {
    /// A single percentage value from 0.0 to 1.0
    SinglePercent(PercentTrack),
    /// A single rotation value from -1.0 to 1.0 (negative CCW, positive CW)
    SingleRotation(RotationTrack),
    /// A 3 Dimensional percentage from (0.0,0.0,0.0) to (1.0,1.0,1.0)
    D3Percent(D3PercentTrack),
    /// A 2 Dimensional rotation value from -1.0 to 1.0 (negative CCW, positive CW)
    D2Rotation(D2RotationTrack),
}

impl FeatureTrackDetail {
    pub fn empty_from_feature_type(feature_type: &FixtureFeatureType) -> FeatureTrackDetail {
        match feature_type {
            FixtureFeatureType::Dimmer => Self::SinglePercent(PercentTrack { values: vec![] }),
            FixtureFeatureType::White => Self::SinglePercent(PercentTrack { values: vec![] }),
            FixtureFeatureType::Rgb => Self::D3Percent(D3PercentTrack { values: vec![] }),
            FixtureFeatureType::Rotation => Self::SingleRotation(RotationTrack { values: vec![] }),
            FixtureFeatureType::PanTilt => Self::D2Rotation(D2RotationTrack { values: vec![] }),
            FixtureFeatureType::Amber => Self::SinglePercent(PercentTrack { values: vec![] }),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct PercentTrack {
    pub values: Vec<PercentageKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct D3PercentTrack {
    pub values: Vec<D3PercentageKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct RotationTrack {
    pub values: Vec<RotationKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct D2RotationTrack {
    pub values: Vec<D2RotationKey>,
}
