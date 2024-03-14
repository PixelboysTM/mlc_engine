use chrono::Duration;
use serde_with::serde_as;
use serde_with::{formats::Flexible, DurationSecondsWithFrac};

use crate::utils::easing::{Easing, EasingType};

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
pub struct D3PercentageKey {
    #[serde(alias = "r")]
    pub x: f32,
    #[serde(alias = "g")]
    pub y: f32,
    #[serde(alias = "b")]
    pub z: f32,
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

pub trait Key {
    type Value: Clone;
    fn time(&self) -> Duration;
    fn value(&self) -> Self::Value;

    fn easing(&self) -> Easing;
}

impl<K: Key> Key for &K {
    type Value = K::Value;

    fn time(&self) -> Duration {
        K::time(self)
    }

    fn value(&self) -> Self::Value {
        K::value(self)
    }

    fn easing(&self) -> Easing {
        K::easing(self)
    }
}

impl Key for FaderKey {
    type Value = u8;
    fn time(&self) -> Duration {
        self.start_time
    }
    fn value(&self) -> Self::Value {
        self.value
    }
    fn easing(&self) -> Easing {
        Easing::new(EasingType::Const, EasingType::Const)
    }
}

impl Key for PercentageKey {
    type Value = f32;
    fn time(&self) -> Duration {
        self.start_time
    }
    fn value(&self) -> Self::Value {
        self.value
    }
    fn easing(&self) -> Easing {
        self.easing
    }
}

impl Key for RotationKey {
    type Value = f32;
    fn time(&self) -> Duration {
        self.start_time
    }
    fn value(&self) -> Self::Value {
        self.value
    }
    fn easing(&self) -> Easing {
        self.easing
    }
}

impl Key for D3PercentageKey {
    type Value = (f32, f32, f32);

    fn time(&self) -> Duration {
        self.start_time
    }

    fn value(&self) -> Self::Value {
        (self.x, self.y, self.z)
    }

    fn easing(&self) -> Easing {
        self.easing
    }
}
