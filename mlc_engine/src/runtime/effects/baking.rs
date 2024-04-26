use std::collections::HashMap;
use std::slice::Iter;
use std::time::Instant;

use chrono::Duration;
use rocket::futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use rocket::futures::{SinkExt, StreamExt};
use rocket::tokio::task::JoinHandle;
use tap::Tap;

use feature_tile_to_raw as to_raw;
use mlc_common::easing::{Easing, EasingType};
use mlc_common::effect::*;
use mlc_common::fixture::FaderAddress;
use mlc_common::patched::feature::FixtureFeature;
use mlc_common::patched::PatchedFixture;

use crate::fixture::feature::feature_tile_to_raw;

pub type BakedEffectCue = Vec<(Duration, u8)>;
pub type BakedFixtureData = Vec<PatchedFixture>;

pub enum BakingRequest {
    Bake(Effect),
    Fixtures(BakedFixtureData),
    Shutdown,
}

pub struct EffectBaker {
    pub task_sender: UnboundedSender<BakingRequest>,
    pub effect_recv: UnboundedReceiver<(EffectId, BakedEffect)>,
    pub join_handle: JoinHandle<()>,
}

pub fn startup_effect_baker(patched_fixtures: Vec<PatchedFixture>) -> EffectBaker {
    let (task_sender, mut task_receiver) = mpsc::unbounded::<BakingRequest>();
    let (mut effect_sender, effect_recv) = mpsc::unbounded::<(EffectId, BakedEffect)>();

    let handle = rocket::tokio::task::spawn(async move {
        let mut fixtures = patched_fixtures;
        while let Some(task) = task_receiver.next().await {
            match task {
                BakingRequest::Fixtures(fs) => fixtures = fs,
                BakingRequest::Bake(effect) => {
                    let time = Instant::now();
                    let backed = bake(&effect, &fixtures).await;
                    let _ = effect_sender.send((effect.id, backed)).await;
                    println!("Baked effect {} in {:?}", effect.id, time.elapsed())
                }
                BakingRequest::Shutdown => break,
            }
        }
    });

    EffectBaker {
        task_sender,
        effect_recv,
        join_handle: handle,
    }
}

#[derive(Debug)]
pub struct BakedEffect {
    pub(super) faders: HashMap<FaderAddress, Vec<(Duration, u8)>>,
    pub(super) max_time: Duration,
    pub(super) looping: bool,
}

pub(crate) async fn bake(effect: &Effect, patched_fixtures: &BakedFixtureData) -> BakedEffect {
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
    track: &mlc_common::effect::FeatureTrack,
    max_time: &Duration,
    fixtures: &BakedFixtureData,
) -> Vec<(FaderAddress, BakedEffectCue)> {
    let mut baked_tracks = vec![];

    for f_id in &track.fixtures {
        let patched: Option<_> = fixtures.iter().find(|u| &u.id == f_id);
        let mut cues = if let Some(fixture) = patched {
            let feature = fixture
                .features
                .iter()
                .find(|feat| feat.name() == track.feature);
            if let Some(feature) = feature {
                match &track.detail {
                    FeatureTrackDetail::SinglePercent(t) => {
                        bake_feature_track_single_percent(t, max_time, feature, &track.resolution)
                            .await
                    }
                    FeatureTrackDetail::D3Percent(t) => {
                        bake_feature_track_three_percent(t, max_time, feature, &track.resolution)
                            .await
                    }
                    FeatureTrackDetail::SingleRotation(t) => {
                        bake_feature_track_single_rotation(t, max_time, feature, &track.resolution)
                            .await
                    }
                    FeatureTrackDetail::D2Rotation(t) => {
                        bake_feature_track_d2_rotation(t, max_time, feature, &track.resolution)
                            .await
                    }
                }
            } else {
                println!(
                    "Couldn't find feature for baking: {:?}, {:?}",
                    track.feature, fixture.features
                );
                vec![]
            }
        } else {
            println!("Fixture not found for baking");
            vec![]
        };
        baked_tracks.append(&mut cues);
    }

    baked_tracks
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

    let vals = get_valid_keys_sorted(t.values.iter(), max_time);

    let time_steps = build_time_steps(resolution, max_time, &vals, 0.0, 0.0, |in_v, out_v, val| {
        in_v + (out_v - in_v) * val
    });

    convert_to_cues::<PercentageKey, _, 1>(&time_steps, |v| [to_raw(feature_tile, v)])
}

async fn bake_feature_track_single_rotation(
    t: &RotationTrack,
    max_time: &Duration,
    fixture_feature: &FixtureFeature,
    resolution: &Duration,
) -> Vec<(FaderAddress, BakedEffectCue)> {
    let (feature_tile_cw, feature_tile_ccw) = match fixture_feature {
        FixtureFeature::Rotation(r) => (&r.cw, &r.ccw),
        _ => {
            eprintln!(
                "Baking Single Rotation for Feature: {} not supported",
                fixture_feature.name()
            );
            return vec![];
        }
    };

    let vals = get_valid_keys_sorted(t.values.iter(), max_time);

    let time_steps = build_time_steps(resolution, max_time, &vals, 0.0, 0.0, |in_v, out_v, val| {
        in_v + (out_v - in_v) * val
    });

    convert_to_cues::<RotationKey, _, 1>(&time_steps, |v| {
        [if v >= &0.0 {
            to_raw(feature_tile_cw, &(*v / 1.0))
        } else {
            to_raw(feature_tile_ccw, &(v.abs() / 1.0))
        }]
    })
}

async fn bake_feature_track_d2_rotation(
    t: &D2RotationTrack,
    max_time: &Duration,
    fixture_feature: &FixtureFeature,
    resolution: &Duration,
) -> Vec<(FaderAddress, BakedEffectCue)> {
    let (pan, tilt) = match fixture_feature {
        FixtureFeature::PanTilt(r) => (&r.pan, &r.tilt),
        _ => {
            eprintln!(
                "Baking Single Rotation for Feature: {} not supported",
                fixture_feature.name()
            );
            return vec![];
        }
    };

    let vals = get_valid_keys_sorted(t.values.iter(), max_time);

    let time_steps = build_time_steps(
        resolution,
        max_time,
        &vals,
        (0.0, 0.0),
        (0.0, 0.0),
        |(in_x, in_y), (out_x, out_y), val| {
            (in_x + (out_x - in_x) * val, in_y + (out_y - in_y) * val)
        },
    );

    convert_to_cues::<D2RotationKey, _, 2>(&time_steps, |v| [to_raw(pan, &v.0), to_raw(tilt, &v.1)])
}

async fn bake_feature_track_three_percent(
    t: &D3PercentTrack,
    max_time: &Duration,
    fixture_feature: &FixtureFeature,
    resolution: &Duration,
) -> Vec<(FaderAddress, BakedEffectCue)> {
    let (d1, d2, d3) = match fixture_feature {
        FixtureFeature::Rgb(rgb) => (&rgb.red, &rgb.green, &rgb.blue),
        _ => {
            eprintln!(
                "Baking D3 Percent for Feature: {} not supported",
                fixture_feature.name()
            );
            return vec![];
        }
    };

    let vals = get_valid_keys_sorted(t.values.iter(), max_time);

    let time_steps = build_time_steps(
        resolution,
        max_time,
        &vals,
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        |(in_x, in_y, in_z), (out_x, out_y, out_z), val| {
            (
                in_x + (out_x - in_x) * val,
                in_y + (out_y - in_y) * val,
                in_z + (out_z - in_z) * val,
            )
        },
    );

    convert_to_cues::<D3PercentageKey, _, 3>(&time_steps, |v| {
        [to_raw(d1, &v.0), to_raw(d2, &v.1), to_raw(d3, &v.2)]
    })
}

fn out_time_filter<F: Key>(max_time: &Duration) -> Box<dyn Fn(&&F) -> bool + '_> {
    let zero = Duration::milliseconds(0);
    Box::new(move |k: &&F| &k.time() <= &max_time && k.time() >= zero)
}

fn make_resolution_times(resolution: &Duration, max: &Duration) -> ResolutionTimeIter {
    ResolutionTimeIter {
        current: Duration::seconds(0),
        resolution: *resolution,
        max: *max,
    }
}

fn get_valid_keys_sorted<'a, K: Key>(iter: Iter<'a, K>, max_time: &'a Duration) -> Vec<&'a K> {
    iter.filter(out_time_filter(max_time))
        .collect::<Vec<_>>()
        .tap_mut(|v| v.sort_by_key(|k| k.time()))
}

fn find_in_out_keys<'a, K: Key>(
    vals: &'a Vec<K>,
    time: &Duration,
) -> (Option<&'a K>, Option<&'a K>) {
    let mut in_key = None;
    let mut out_key = None;

    'finder: for val in vals {
        if &val.time() <= time {
            in_key = Some(val);
        } else {
            out_key = Some(val);
            break 'finder;
        }
    }

    (in_key, out_key)
}

fn split_in_key<K: Key>(
    key: Option<&K>,
    vals: &[K],
    default_value: K::Value,
) -> (Duration, K::Value, EasingType) {
    key.map(|k| (k.time(), k.value(), k.easing().out_type))
        .unwrap_or((
            Duration::seconds(0),
            vals.first().map(|k| k.value()).unwrap_or(default_value),
            EasingType::Const,
        ))
}

fn split_out_key<K: Key>(
    key: Option<&K>,
    vals: &[K],
    max_time: &Duration,
    default_value: K::Value,
) -> (Duration, K::Value, EasingType) {
    key.map(|k| (k.time(), k.value(), k.easing().in_type))
        .unwrap_or((
            *max_time,
            vals.last().map(|k| k.value()).unwrap_or(default_value),
            EasingType::Const,
        ))
}

fn get_t(in_t: Duration, out_t: Duration, time: Duration) -> f32 {
    let range = (out_t - in_t).num_milliseconds();
    let time_point = (time - in_t).num_milliseconds();

    time_point as f32 / range as f32
}

fn convert_to_cues<K, F, const N: usize>(
    time_steps: &[(Duration, K::Value)],
    to_raw_val_fun: F,
) -> Vec<(FaderAddress, Vec<(Duration, u8)>)>
where
    K: Key,
    F: Fn(&K::Value) -> [Vec<(FaderAddress, u8)>; N],
{
    if time_steps.is_empty() {
        return vec![];
    }

    let steps = time_steps
        .iter()
        .map(|(t, v)| {
            (
                t,
                to_raw_val_fun(v)
                    .iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut faders = steps[0]
        .1
        .iter()
        .map(|(f, _)| (*f, vec![]))
        .collect::<Vec<_>>();

    for (d, fs) in steps {
        for (i, (_, v)) in fs.iter().enumerate() {
            faders[i].1.push((*d, *v));
        }
    }

    faders
}

fn build_time_steps<K: Key, F>(
    resolution: &Duration,
    max_time: &Duration,
    vals: &Vec<&K>,
    in_default: K::Value,
    out_default: K::Value,
    value_producer_fn: F,
) -> Vec<(Duration, K::Value)>
where
    F: Fn(K::Value, K::Value, f32) -> K::Value,
{
    let mut time_steps = Vec::new();

    for time in make_resolution_times(resolution, max_time) {
        let (in_key, out_key) = find_in_out_keys(vals, &time);

        let (in_t, in_v, left_e) = split_in_key(in_key, vals, in_default.clone());
        let (out_t, out_v, right_e) = split_out_key(out_key, vals, max_time, out_default.clone());

        let t = get_t(in_t, out_t, time);
        let easing = Easing::new(left_e, right_e);
        let val = easing.eval(t).min(1.0).max(0.0);

        let value = value_producer_fn(in_v, out_v, val);

        time_steps.push((time, value));
    }

    time_steps
}

struct ResolutionTimeIter {
    current: Duration,
    resolution: Duration,
    max: Duration,
}

impl Iterator for ResolutionTimeIter {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.current;

        self.current += self.resolution;

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
