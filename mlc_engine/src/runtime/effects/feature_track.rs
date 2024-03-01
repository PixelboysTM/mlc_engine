use chrono::Duration;
use serde_with::{DurationSecondsWithFrac, formats::Flexible};
use serde_with::formats::PreferOne;
use serde_with::OneOrMany;
use serde_with::serde_as;
use mlc_common::patched::feature::FixtureFeatureType;
use mlc_common::patched::FixtureId;

use crate::runtime::effects::track_key::{D3PercentageKey, PercentageKey, RotationKey};

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct FeatureTrack {
    #[serde(alias = "fixture")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub(super) fixtures: Vec<FixtureId>,
    pub(super) feature: FixtureFeatureType,
    pub(super) detail: FeatureTrackDetail,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    pub(super) resolution: Duration,
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
    pub(super) values: Vec<PercentageKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct D3PercentTrack {
    pub(super) values: Vec<D3PercentageKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RotationTrack {
    pub(super) values: Vec<RotationKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct D2RotationTrack {
    pub(super) values: Vec<(RotationKey, RotationKey)>,
}
