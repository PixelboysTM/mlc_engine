use std::fmt::Display;
use std::thread::Scope;

use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use mlc_common::effect::{
    D2RotationKey, D3PercentageKey, FaderKey, Key, PercentageKey, RotationKey, Track,
};
use mlc_common::effect::{Effect as FEffect, FeatureTrackDetail};
use mlc_common::fixture::FaderAddress;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlElement;

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
                            FeatureTrackDetail::SinglePercent(_) => rsx!{FeatureTrackEditor{}},
                            FeatureTrackDetail::SingleRotation(_) => rsx!(""),
                            FeatureTrackDetail::D3Percent(_) => rsx!(""),
                            FeatureTrackDetail::D2Rotation(_) => rsx!(""),
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
fn FeatureTrackEditor<K: Clone + Key + PartialEq + 'static>(
    track_key: K,
    onchnage: EventHandler<K::Value>,
) -> Element {
}

#[component]
fn KeyEditorOld(px: f64, py: f64, children: Element, onclose: EventHandler) -> Element {
    rsx! {
        div {
            class: "key-edit",
            style: "--px: {px}px; --py: {py}px;",
            tabindex: -1,
            onmounted: move |e| {
                let _ = e.set_focus(true);
            },
            onfocusout: move |e| {
                let we = e.web_event();
                if let Some(current_target) = we
                    .current_target()
                    .and_then(|c| c.dyn_ref::<HtmlElement>().cloned())
                {
                    if let Some(related_target) = we
                        .related_target()
                        .and_then(|c| c.dyn_ref::<HtmlElement>().cloned())
                    {
                        let c = current_target.contains(Some(&related_target));
                        if c {
                            return;
                        }
                    }
                }
                onclose.call(());
            },
            onclick: move |e| {
                e.stop_propagation();
            },
            oncontextmenu: move |e| {
                e.stop_propagation();
            },
            {children}
        }
    }
}

pub trait DrawKeyWidget<T> {
    fn draw_widget<F>(&self, onchange: F) -> Element
    where
        F: FnMut(T) + 'static;
}

impl DrawKeyWidget<<PercentageKey as mlc_common::effect::Key>::Value> for PercentageKey {
    fn draw_widget<F>(&self, mut onchange: F) -> Element
    where
        F: FnMut(<PercentageKey as mlc_common::effect::Key>::Value) + 'static,
    {
        rsx! {
            div { style: "min-height: 12rem; min-width: 3rem",
                utils::Slider {
                    initial: self.value(),
                    onchange: move |v| {
                        onchange(v);
                    }
                }
            }
        }
    }
}

impl DrawKeyWidget<<D3PercentageKey as mlc_common::effect::Key>::Value> for D3PercentageKey {
    fn draw_widget<F>(&self, mut onchange: F) -> Element
    where
        F: FnMut(<D3PercentageKey as mlc_common::effect::Key>::Value) + 'static,
    {
        rsx! {
            div { style: "min-width: 12rem; min-height: 12rem;",
                utils::RgbWidget {
                    initial: self.value(),
                    onchange: move |v| {
                        onchange(v);
                    }
                }
            }
        }
    }
}

impl DrawKeyWidget<<D2RotationKey as mlc_common::effect::Key>::Value> for D2RotationKey {
    fn draw_widget<F>(&self, mut onchange: F) -> Element
    where
        F: FnMut(<D2RotationKey as mlc_common::effect::Key>::Value) + 'static,
    {
        rsx! {
            div { style: "min-width: 14rem; min-height: 14rem;",
                utils::PanTiltWidget {
                    initial: self.value(),
                    onchange: move |v| {
                        onchange(v);
                    }
                }
            }
        }
    }
}

impl DrawKeyWidget<<RotationKey as mlc_common::effect::Key>::Value> for RotationKey {
    fn draw_widget<F>(&self, _onchange: F) -> Element
    where
        F: FnMut(<RotationKey as mlc_common::effect::Key>::Value) + 'static,
    {
        rsx! { "Unimplemented" }
    }
}
