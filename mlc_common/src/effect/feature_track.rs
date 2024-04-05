use ::serde_with::{DurationSecondsWithFrac, formats::Flexible, formats::PreferOne, OneOrMany};
use chrono::Duration;
use serde_with::serde_as;

use crate::effect::track_key::{D3PercentageKey, PercentageKey, RotationKey};
use crate::patched::feature::FixtureFeatureType;
use crate::patched::FixtureId;

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct FeatureTrack {
    #[serde(alias = "fixture")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub fixtures: Vec<FixtureId>,
    pub feature: FixtureFeatureType,
    pub detail: FeatureTrackDetail,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    pub resolution: Duration,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PercentTrack {
    pub values: Vec<PercentageKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct D3PercentTrack {
    pub values: Vec<D3PercentageKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RotationTrack {
    pub values: Vec<RotationKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct D2RotationTrack {
    pub values: Vec<(RotationKey, RotationKey)>,
}
