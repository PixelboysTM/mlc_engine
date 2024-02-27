use chrono::Duration;
use serde_with::{DurationSecondsWithFrac, formats::Flexible};
use serde_with::serde_as;

use crate::fixture::feature::FixtureFeatureType;
use crate::fixture::FixtureId;
use crate::runtime::effects::track_key::{PercentageKey, RotationKey};

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct FeatureTrack {
    pub(super) fixture: FixtureId,
    pub(super) feature: FixtureFeatureType,
    pub(super) detail: FeatureTrackDetail,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    pub(super) resolution: Duration,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum FeatureTrackDetail {
    SinglePercent(PercentTrack),
    D3Percent(D3PercentTrack),
    SingleRotation(RotationTrack),
    D2Rotation(D2RotationTrack),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PercentTrack {
    pub(super) values: Vec<PercentageKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct D3PercentTrack {
    pub(super) values: Vec<(PercentageKey, PercentTrack, PercentTrack)>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RotationTrack {
    pub(super) values: Vec<RotationKey>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct D2RotationTrack {
    pub(super) min_x: f32,
    pub(super) max_x: f32,
    pub(super) min_y: f32,
    pub(super) max_y: f32,
    pub(super) values: Vec<(RotationKey, RotationKey)>,
}
