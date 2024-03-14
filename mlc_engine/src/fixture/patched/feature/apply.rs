use crate::runtime::RuntimeData;
use mlc_common::patched::feature::{
    Dimmer, FeatureSetRequest, FeatureTile, FixtureFeature, PanTilt, Rgb, Rotation,
};

pub trait ApplyFeature {
    async fn apply(&self, req: FeatureSetRequest, runtime_data: &RuntimeData);
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
            FeatureSetRequest::Amber { value } => {
                if let Some(d) = find_amber(self) {
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
            FeatureSetRequest::PanTilt { pan, tilt } => {
                if let Some(pantilt) = find_pantilt(self) {
                    update_values(&[(pantilt.pan, pan), (pantilt.tilt, tilt)], runtime_data).await;
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
        let v = tile.to_raw(raw_v);
        for (f, v) in v {
            universes.push(f.universe);
            channels.push(f.address);
            values.push(v);
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

fn find_amber(features: &[FixtureFeature]) -> Option<Dimmer> {
    for f in features {
        match f {
            FixtureFeature::Amber(d) => return Some(d.clone()),
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

fn find_pantilt(features: &[FixtureFeature]) -> Option<PanTilt> {
    for f in features {
        match f {
            FixtureFeature::PanTilt(d) => return Some(d.clone()),
            _ => continue,
        }
    }

    None
}
