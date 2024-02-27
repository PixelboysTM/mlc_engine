use chrono::Duration;
use serde_with::serde_as;
use serde_with::{formats::Flexible, DurationSecondsWithFrac};
use crate::utils::easing::Easing;

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct FaderKey {
    pub value: u8,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    pub start_time: Duration,
}

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PercentageKey {
    pub value: f32,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    pub start_time: Duration,
    pub easing: Easing,
}

#[serde_as]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RotationKey {
    pub value: f32,
    #[serde_as(as = "DurationSecondsWithFrac<f64, Flexible>")]
    pub start_time: Duration,
    pub easing: Easing,
}

pub trait Key{
    fn time(&self) -> &Duration;
}

impl Key for FaderKey {
    fn time(&self) -> &Duration {
        &self.start_time
    }
}

impl Key for PercentageKey {
    fn time(&self) -> &Duration {
        &self.start_time
    }
}

impl Key for RotationKey {
    fn time(&self) -> &Duration {
        &self.start_time
    }
}
