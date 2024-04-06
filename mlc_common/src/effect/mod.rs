use chrono::Duration;
use serde_with::{DurationSecondsWithFrac, formats::Flexible};
use serde_with::serde_as;

pub use feature_track::*;
pub use track_key::*;

use crate::fixture::FaderAddress;

mod track_key;
mod feature_track;
pub mod rest;


#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Effect {
    pub id: uuid::Uuid,
    pub name: String,
    pub looping: bool,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    pub duration: Duration,
    pub tracks: Vec<Track>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum Track {
    FaderTrack(FaderTrack),
    FeatureTrack(FeatureTrack),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct FaderTrack {
    pub address: FaderAddress,
    pub values: Vec<FaderKey>,
}