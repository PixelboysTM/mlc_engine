use crate::utils::easing::EasingType;

#[macro_export]
macro_rules! send {
    ($info:expr, $msg:expr) => {
        let _ = $info.send($msg);
    };
}

pub mod easing {

    #[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
    pub enum EasingType {
        Linear,
        Sine,
        Cubic,
        Quint,
        Circ,
        Elastic,
        Quad,
        Quart,
        Expo,
        Back,
        Bounce,
    }

    #[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
    pub struct Easing {
        pub in_type: EasingType,
        pub out_type: EasingType,
    }

    impl Easing {
        pub fn new(left: EasingType, right: EasingType) -> Self {
            Easing {
                in_type: left,
                out_type: right,
            }
        }

        pub fn eval(&self, t: f32) -> f32 {
            let t = t.max(0.0).min(1.0);
            if t < 0.5 {
                self.in_type.val_left(t * 2.0) * 0.5
            } else {
                0.5 + self.out_type.val_right((t - 0.5) * 2.0) * 0.5
            }
        }
    }
}

impl EasingType {
    /// Value of right side of curve
    fn val_right(&self, t: f32) -> f32 {
        use std::f32::consts::PI;

        let t = t.max(0.0).min(1.0);

        match self {
            EasingType::Linear => t,
            EasingType::Sine => ((t * PI) / 2.0).sin(), //1.0 - ((t * PI) / 2.0).sin(),
            EasingType::Cubic => 1.0 - (1.0 - t).powi(3),
            EasingType::Quint => 1.0 - (1.0 - t).powi(5),
            EasingType::Circ => (1.0 - (t - 1.0).powi(2)).sqrt(),
            EasingType::Elastic => {
                let c4 = (2.0 * PI) / 3.0;
                if t <= f32::EPSILON {
                    0.0
                } else if t >= 1.0 - f32::EPSILON {
                    1.0
                } else {
                    (2.0_f32).powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
            EasingType::Quad => 1.0 - (1.0 - t) * (1.0 - t),
            EasingType::Quart => 1.0 - (1.0 - t).powi(4),
            EasingType::Expo => {
                if t >= 1.0 - f32::EPSILON {
                    1.0
                } else {
                    1.0 - 2.0_f32.powf(-10.0 * t)
                }
            }
            EasingType::Back => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) - c1 * (t - 1.0).powi(2)
            }
            EasingType::Bounce => {
                let n1 = 7.5625;
                let d1 = 2.75;

                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    n1 * (t - 1.5 / d1) * (t - 1.5) + 0.75
                } else if t < 2.5 / d1 {
                    n1 * (t - 2.25 / d1) * (t - 2.25) + 0.9375
                } else {
                    n1 * (t - 2.625 / d1) * (t - 2.625) + 0.984375
                }
            }
        }
    }

    /// Value of left side of curve
    fn val_left(&self, t: f32) -> f32 {
        use std::f32::consts::PI;
        let t = t.max(0.0).min(1.0);

        match self {
            EasingType::Linear => t,
            EasingType::Sine => ((t * PI) / 2.0 - PI / 2.0).sin() + 1.0,
            EasingType::Cubic => t * t * t,
            EasingType::Quint => t * t * t * t * t,
            EasingType::Circ => 1.0 - (1.0 - t.powi(2)).sqrt(),
            EasingType::Elastic => {
                let c4 = (2.0 * PI) / 3.0;
                if t <= f32::EPSILON {
                    0.0
                } else if t >= 1.0 - f32::EPSILON {
                    1.0
                } else {
                    -(2.0_f32).powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()
                }
            }
            EasingType::Quad => t * t,
            EasingType::Quart => t * t * t * t,
            EasingType::Expo => {
                if t <= f32::EPSILON {
                    0.0
                } else {
                    2.0_f32.powf(10.0 * t - 10.0)
                }
            }
            EasingType::Back => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            EasingType::Bounce => 1.0 - EasingType::Bounce.val_right(1.0 - t),
        }
    }
}
