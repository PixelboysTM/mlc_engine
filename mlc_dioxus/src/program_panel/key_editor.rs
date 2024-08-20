use dioxus::prelude::*;
use mlc_common::effect::{
    D2RotationKey, D2RotationTrack, D3PercentTrack, D3PercentageKey, FaderKey, FeatureTrack, Key,
    PercentTrack, PercentageKey, RotationKey, RotationTrack, Track,
};
use mlc_common::effect::{Effect as FEffect, FeatureTrackDetail};
use mlc_common::fixture::FaderAddress;

use crate::configure_panel::{make_three_digit, Fader};
use crate::program_panel::EffectInvalidate;
use crate::utils;

#[component]
pub fn KeyFrameInspector() -> Element {
    let current_keyframe = use_context::<Signal<Option<(usize, usize)>>>();
    let mut current_effect = use_context::<Signal<Option<FEffect>>>();
    let effect_invalidator: Coroutine<EffectInvalidate> = use_coroutine_handle();

    rsx! {
        match current_keyframe() {
            Some((track_index, key)) => {
                match current_effect().expect("Must be valid").tracks[track_index].clone() {
                    Track::FaderTrack(track) => {
                        rsx!{
                            FaderTrackEditor {
                                fader_key: track.values[key].clone(),
                                addr: track.address,
                                change: move |v| {
                                    let mut oe = current_effect.write();
                                    let e = oe.as_mut().expect("Must be");
                                    match &mut e.tracks[track_index] {
                                        Track::FaderTrack(t) => t.values[key].value = v,
                                        Track::FeatureTrack(_) => unreachable!(),
                                    }
                                    effect_invalidator.send(EffectInvalidate);
                                }
                            }
                        }
                    }
                    Track::FeatureTrack(track) => {
                        match track.detail {
                            FeatureTrackDetail::SinglePercent(s) => rsx!{
                                FeatureTrackEditor{track_key: s.values[key].clone(), change: move |v| {let mut oe = current_effect.write();
                                    let e = oe.as_mut().expect("Must be");
                                    match &mut e.tracks[track_index] {
                                        Track::FeatureTrack(FeatureTrack{detail: FeatureTrackDetail::SinglePercent(PercentTrack{values}), ..}) => values[key].value = v,
                                        Track::FaderTrack(_) => unreachable!(),
                                        Track::FeatureTrack(_) => unreachable!(),
                                    }
                                    effect_invalidator.send(EffectInvalidate);}}
                                },
                            FeatureTrackDetail::SingleRotation(s) => rsx!{
                                FeatureTrackEditor{track_key: s.values[key].clone(), change: move |v| {let mut oe = current_effect.write();
                                    let e = oe.as_mut().expect("Must be");
                                    match &mut e.tracks[track_index] {
                                        Track::FeatureTrack(FeatureTrack{detail: FeatureTrackDetail::SingleRotation(RotationTrack{values}), ..}) => values[key].value = v,
                                        Track::FaderTrack(_) => unreachable!(),
                                        Track::FeatureTrack(_) => unreachable!(),
                                    }
                                    effect_invalidator.send(EffectInvalidate);}}
                                },
                            FeatureTrackDetail::D3Percent(s) => rsx!{
                                FeatureTrackEditor{track_key: s.values[key].clone(), change: move |(x,y,z)| {let mut oe = current_effect.write();
                                    let e = oe.as_mut().expect("Must be");
                                    match &mut e.tracks[track_index] {
                                        Track::FeatureTrack(FeatureTrack{detail: FeatureTrackDetail::D3Percent(D3PercentTrack{values}), ..}) => {values[key].x = x;values[key].y = y;values[key].z = z;},
                                        Track::FaderTrack(_) => unreachable!(),
                                        Track::FeatureTrack(_) => unreachable!(),
                                    }
                                    effect_invalidator.send(EffectInvalidate);}}
                                },
                            FeatureTrackDetail::D2Rotation(s) => rsx!{
                                FeatureTrackEditor{track_key: s.values[key].clone(), change: move |(x,y)| {let mut oe = current_effect.write();
                                    let e = oe.as_mut().expect("Must be");
                                    match &mut e.tracks[track_index] {
                                        Track::FeatureTrack(FeatureTrack{detail: FeatureTrackDetail::D2Rotation(D2RotationTrack{values}), ..}) => {values[key].x = x; values[key].y = y;},
                                        Track::FaderTrack(_) => unreachable!(),
                                        Track::FeatureTrack(_) => unreachable!(),
                                    }
                                    effect_invalidator.send(EffectInvalidate);}}
                                },
                        }
                    }
                }
            }
            None => {
                rsx! {
                    p {
                        class: "inspect-none",
                        "Select a Keyframe to edit it"
                    }
                }
            }
        }
    }
}

#[component]
fn FaderTrackEditor(fader_key: FaderKey, change: EventHandler<u8>, addr: FaderAddress) -> Element {
    rsx! {
        div { class: "editor fader",
            Fader {
                value: fader_key.value,
                id: make_three_digit(addr.address.i() as u16),
                onchange: move |v| {
                    change.call(v);
                }
            }
        }
    }
}

#[component]
fn FeatureTrackEditor<K: DrawKeyWidget + Clone + PartialEq + 'static>(
    track_key: K,
    change: EventHandler<K::Value>,
) -> Element {
    rsx! {
        div { class: "editor feature",
            K::draw_widget {
                value: track_key.value(),
                change: move |v| {
                    change.call(v);
                }
            }
        }
    }
}

#[derive(Props, Clone)]
pub struct DKWProps<K: Key + Clone + PartialEq + 'static, F: Fn(K::Value) + 'static> {
    value: K::Value,
    change: F,
}

impl<K, F1, F2> PartialEq<DKWProps<K, F2>> for DKWProps<K, F1>
where
    K: Key + Clone + PartialEq + 'static,
    F1: Fn(K::Value) + 'static,
    F2: Fn(K::Value) + 'static,
{
    fn eq(&self, other: &DKWProps<K, F2>) -> bool {
        self.value == other.value
    }
}

pub trait DrawKeyWidget: Key + Clone + PartialEq + 'static {
    fn draw_widget<F>(props: DKWProps<Self, F>) -> Element
    where
        F: Fn(Self::Value) + 'static;
}

impl DrawKeyWidget for PercentageKey {
    fn draw_widget<F>(props: DKWProps<Self, F>) -> Element
    where
        F: Fn(Self::Value) + 'static,
    {
        rsx! {
            utils::Slider {
                initial: props.value,
                onchange: move |v| {
                    (props.change)(v);
                }
            }
        }
    }
}

impl DrawKeyWidget for RotationKey {
    fn draw_widget<F>(_props: DKWProps<Self, F>) -> Element
    where
        F: Fn(Self::Value) + 'static,
    {
        rsx! { "UNIMPLEMENTED" }
    }
}

impl DrawKeyWidget for D2RotationKey {
    fn draw_widget<F>(props: DKWProps<Self, F>) -> Element
    where
        F: Fn(Self::Value) + 'static,
    {
        rsx! {
            utils::PanTiltWidget {
                initial: props.value,
                onchange: move |v| {
                    (props.change)(v);
                }
            }
        }
    }
}

impl DrawKeyWidget for D3PercentageKey {
    fn draw_widget<F>(props: DKWProps<Self, F>) -> Element
    where
        F: Fn(Self::Value) + 'static,
    {
        log::info!("Updating value: {:?}", props.value);
        rsx! {
            utils::RgbWidget {
                initial: props.value,
                onchange: move |v| {
                    (props.change)(v);
                }
            }
        }
    }
}
