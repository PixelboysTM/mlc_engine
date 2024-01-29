use crate::runtime::{RuntimeData, ToFaderValue};

use super::{Dimmer, FixtureFeature, Rgb};

pub trait ApplyFeature {
    async fn apply(&self, req: FeatureSetRequest, runtime_data: &RuntimeData);
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum FeatureSetRequest {
    Dimmer { value: f32 },
    Rgb { red: f32, green: f32, blue: f32 },
    GetAvailableFeatures,
}

impl ApplyFeature for Vec<FixtureFeature> {
    async fn apply(&self, req: FeatureSetRequest, runtime_data: &RuntimeData) {
        match req {
            FeatureSetRequest::Dimmer { value } => {
                if let Some(d) = find_dimmer(self) {
                    runtime_data
                        .set_value(
                            d.dimmer.fader.universe,
                            d.dimmer.fader.address,
                            value.to_fader_value_range(&d.dimmer.range),
                        )
                        .await;
                }
            }
            FeatureSetRequest::Rgb { red, green, blue } => {
                if let Some(rgb) = find_rgb(self) {
                    runtime_data
                        .set_values(
                            vec![
                                rgb.red.fader.universe,
                                rgb.green.fader.universe,
                                rgb.blue.fader.universe,
                            ],
                            vec![
                                rgb.red.fader.address,
                                rgb.green.fader.address,
                                rgb.blue.fader.address,
                            ],
                            vec![
                                red.to_fader_value_range(&rgb.red.range),
                                green.to_fader_value_range(&rgb.green.range),
                                blue.to_fader_value_range(&rgb.blue.range),
                            ],
                        )
                        .await;
                }
            }
            FeatureSetRequest::GetAvailableFeatures => {
                eprintln!("Something is not working with your code dumb ass")
            }
        }
    }
}

fn find_dimmer(features: &[FixtureFeature]) -> Option<Dimmer> {
    for f in features {
        match f {
            FixtureFeature::Dimmer(d) => return Some(d.clone()),
            _ => continue,
        }
    }

    None
}
fn find_rgb(features: &[FixtureFeature]) -> Option<Rgb> {
    for f in features {
        match f {
            FixtureFeature::Rgb(d) => return Some(d.clone()),
            _ => continue,
        }
    }

    None
}
