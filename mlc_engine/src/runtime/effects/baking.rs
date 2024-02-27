use std::collections::HashMap;

use chrono::Duration;
use tap::{Pipe, Tap};

use crate::fixture::{FaderAddress, PatchedFixture};
use crate::fixture::feature::FixtureFeature;
use crate::runtime::effects::{Effect, FaderTrack, Track};
use crate::runtime::effects::feature_track::{FeatureTrack, FeatureTrackDetail, PercentTrack};
use crate::runtime::effects::track_key::Key;
use crate::utils::easing::{Easing, EasingType};

pub type BakedEffectCue = Vec<(Duration, u8)>;
pub type BakedFixtureData = Vec<PatchedFixture>;

#[derive(Debug)]
pub struct BakedEffect {
    pub(super) faders: HashMap<FaderAddress, Vec<(Duration, u8)>>,
    pub(super) current_time: Duration,
    pub(super) max_time: Duration,
    pub(super) running: bool,
    pub(super) looping: bool,
}

impl BakedEffect {
    pub(super) fn toggle(&mut self) {
        if self.running {
            self.running = false;
        } else {
            self.current_time = Duration::milliseconds(0);
            self.running = true;
        }
    }
}
pub(crate) async fn bake(effect: &Effect, patched_fixtures: BakedFixtureData) -> BakedEffect {
    let mut faders = HashMap::new();

    for track in &effect.tracks {
        match track {
            Track::FaderTrack(cue) => {
                faders.insert(cue.address, bake_fader_cue(cue, &effect.duration));
            }
            Track::FeatureTrack(cue) => {
                let baked_cues: Vec<(FaderAddress, BakedEffectCue)> =
                    bake_feature_track(cue, &effect.duration, &patched_fixtures).await;
                for (address, baked_cue) in baked_cues {
                    faders.insert(address, baked_cue);
                }
            }
        };
    }

    // println!("{:#?}", &faders);

    BakedEffect {
        faders,
        max_time: effect.duration,
        current_time: Duration::milliseconds(0),
        running: false,
        looping: effect.looping,
    }
}

fn bake_fader_cue(fader_cue: &FaderTrack, max_time: &Duration) -> BakedEffectCue {
    let mut vals: Vec<_> = fader_cue
        .values
        .iter()
        .filter(out_time_filter(max_time))
        .map(|f| (f.start_time, f.value))
        .collect();
    vals.sort_by_key(|k| k.0);

    vals
}

async fn bake_feature_track(
    track: &FeatureTrack,
    max_time: &Duration,
    fixtures: &BakedFixtureData,
) -> Vec<(FaderAddress, BakedEffectCue)> {
    let patched: Option<_> = fixtures.iter().find(|u| u.id == track.fixture);
    if let Some(fixture) = patched {
        let feature = fixture
            .features
            .iter()
            .find(|feat| feat.name() == track.feature);
        if let Some(feature) = feature {
            match &track.detail {
                FeatureTrackDetail::SinglePercent(t) => {
                    bake_feature_track_single_percent(t, max_time, feature, &track.resolution).await
                }
                FeatureTrackDetail::D3Percent(_) => todo!(),
                FeatureTrackDetail::SingleRotation(_) => todo!(),
                FeatureTrackDetail::D2Rotation(_) => todo!(),
            }
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

async fn bake_feature_track_single_percent(
    t: &PercentTrack,
    max_time: &Duration,
    fixture_feature: &FixtureFeature,
    resolution: &Duration,
) -> Vec<(FaderAddress, BakedEffectCue)> {
    let feature_tile = match fixture_feature {
        FixtureFeature::Dimmer(d) => &d.dimmer,
        FixtureFeature::White(w) => &w.dimmer,
        FixtureFeature::Amber(a) => &a.dimmer,
        _ => {
            eprintln!(
                "Baking Single Percent for Feature: {} not supported",
                fixture_feature.name()
            );
            return vec![];
        }
    };

    let mut vals: Vec<_> = t
        .values
        .iter()
        .filter(out_time_filter(max_time))
        .collect::<Vec<_>>()
        .tap_mut(|v| v.sort_by_key(|k| k.start_time));

    let mut time_steps = Vec::new();

    for time in make_resolution_times(resolution, max_time) {
        let mut in_key = None;
        let mut out_key = None;

        'finder: for val in &vals {
            if val.start_time <= time {
                in_key = Some(val);
            } else {
                out_key = Some(val);
                break 'finder;
            }
        }

        let (in_t, in_v, left_e) = in_key
            .map(|k| (k.start_time, k.value, k.easing.out_type))
            .unwrap_or((
                Duration::seconds(0),
                vals.first().map(|k| k.value).unwrap_or(0.0),
                EasingType::Const,
            ));
        let (out_t, out_v, right_e) = out_key
            .map(|k| (k.start_time, k.value, k.easing.in_type))
            .unwrap_or((
                max_time.clone(),
                vals.last().map(|k| k.value).unwrap_or(0.0),
                EasingType::Const,
            ));

        let range = (out_t - in_t).num_milliseconds();
        let time_point = (time - in_t).num_milliseconds();

        let t = time_point as f32 / range as f32;
        let easing = Easing::new(left_e, right_e);
        let val = easing.eval(t).min(1.0).max(0.0);

        let value = in_v + (out_v - in_v) * val;

        time_steps.push((time, value));
    }

    if time_steps.is_empty() {
        return vec![];
    }

    let steps: Vec<_> = time_steps
        .iter()
        .map(|(t, v)| (t, feature_tile.to_raw(v)))
        .collect();

    let mut faders = steps[0]
        .1
        .iter()
        .map(|(f, _)| (*f, vec![]))
        .collect::<Vec<_>>();

    for (d, fs) in steps {
        for (i, (_, v)) in fs.iter().enumerate() {
            faders[i].1.push((d.clone(), *v));
        }
    }

    faders
}

fn out_time_filter<F: Key>(max_time: &Duration) -> Box<dyn Fn(&&F) -> bool + '_> {
    let zero = Duration::milliseconds(0);
    Box::new(move |k: &&F| &k.time() <= &max_time && k.time() >= &zero)
}

#[derive(Clone, serde::Serialize)]
pub enum BakingNotification {
    Started(String),
    Finished(String),
    Misc(String),
}

fn make_resolution_times(resolution: &Duration, max: &Duration) -> ResolutionTimeIter {
    ResolutionTimeIter {
        current: Duration::seconds(0),
        resolution: resolution.clone(),
        max: max.clone(),
    }
}

struct ResolutionTimeIter {
    current: Duration,
    resolution: Duration,
    max: Duration,
}

impl Iterator for ResolutionTimeIter {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.current.clone();

        self.current = self.current + self.resolution;

        if cur > self.max {
            None
        } else {
            Some(cur)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::runtime::effects::baking::ResolutionTimeIter;

    #[test]
    fn resolution_timer_iter() {
        let i = ResolutionTimeIter {
            resolution: Duration::seconds(1),
            current: Duration::seconds(0),
            max: Duration::seconds(5),
        };

        let mut index = 0;
        for d in i {
            assert_eq!(d, Duration::seconds(index));
            index += 1;
        }
    }
}
