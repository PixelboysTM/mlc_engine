use crate::{
    fixture::Value,
    runtime::{RuntimeData, ToFaderValue},
};

use super::{Dimmer, FeatureTile, FixtureFeature, Rgb, Rotation};

pub trait ApplyFeature {
    async fn apply(&self, req: FeatureSetRequest, runtime_data: &RuntimeData);
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum FeatureSetRequest {
    Dimmer { value: f32 },
    Rgb { red: f32, green: f32, blue: f32 },
    White { value: f32 },
    Rotation { value: f32 },
    GetAvailableFeatures,
}

impl ApplyFeature for Vec<FixtureFeature> {
    async fn apply(&self, req: FeatureSetRequest, runtime_data: &RuntimeData) {
        match req {
            FeatureSetRequest::Dimmer { value } => {
                if let Some(d) = find_dimmer(self) {
                    update_values(&[(d.dimmer, value)], runtime_data).await;
                }
            }
            FeatureSetRequest::White { value } => {
                if let Some(d) = find_white(self) {
                    update_values(&[(d.dimmer, value)], runtime_data).await;
                }
            }
            FeatureSetRequest::Rgb { red, green, blue } => {
                if let Some(rgb) = find_rgb(self) {
                    update_values(
                        &[(rgb.red, red), (rgb.blue, blue), (rgb.green, green)],
                        runtime_data,
                    )
                    .await;
                }
            }
            FeatureSetRequest::Rotation { value } => {
                if let Some(rot) = find_rotation(self) {
                    if value > 0.0 {
                        update_values(&[(rot.cw, value.abs())], runtime_data).await;
                    }
                    if value < 0.0 {
                        update_values(&[(rot.ccw, value.abs())], runtime_data).await;
                    }
                }
            }
            FeatureSetRequest::GetAvailableFeatures => {
                eprintln!("Something is not working with your code dumb ass")
            }
        }
    }
}

async fn update_values(ts: &[(FeatureTile, f32)], runtime: &RuntimeData) {
    let mut universes = vec![];
    let mut channels = vec![];
    let mut values = vec![];

    for (tile, raw_v) in ts {
        match tile {
            FeatureTile::Single { fader, range, .. } => {
                let v = raw_v.to_fader_value_range(range);
                universes.push(fader.universe);
                channels.push(fader.address);
                values.push(v);
            }
            FeatureTile::Double {
                fader,
                fader_fine,
                range,
                ..
            } => {
                let (v, f) = raw_v.to_fader_value_range_fine(range);
                universes.push(fader.universe);
                universes.push(fader_fine.universe);
                channels.push(fader.address);
                channels.push(fader_fine.address);
                values.push(v);
                values.push(f);
            }
            FeatureTile::Tripple {
                fader,
                fader_fine,
                fader_grain,
                range,
                ..
            } => {
                let (v, f, g) = raw_v.to_fader_value_range_grain(range);
                universes.push(fader.universe);
                universes.push(fader_fine.universe);
                universes.push(fader_grain.universe);
                channels.push(fader.address);
                channels.push(fader_fine.address);
                channels.push(fader_grain.address);
                values.push(v);
                values.push(f);
                values.push(g);
            }
        }
    }

    if universes.is_empty() {
        return;
    }

    if universes.len() == 1 {
        runtime
            .set_value(universes[0], channels[0], values[0])
            .await;
        return;
    }

    runtime.set_values(universes, channels, values).await;
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
fn find_rotation(features: &[FixtureFeature]) -> Option<Rotation> {
    for f in features {
        match f {
            FixtureFeature::Rotation(d) => return Some(d.clone()),
            _ => continue,
        }
    }

    None
}
fn find_white(features: &[FixtureFeature]) -> Option<Dimmer> {
    for f in features {
        match f {
            FixtureFeature::White(d) => return Some(d.clone()),
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
