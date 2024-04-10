use std::collections::HashSet;
use chrono::Duration;
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use crate::{icons, utils};
use mlc_common::effect::{Effect, FaderTrack, FeatureTrack, FeatureTrackDetail, Track};
use mlc_common::fixture::FaderAddress;
use mlc_common::patched::{FixtureId, UniverseAddress, UniverseId};
use mlc_common::patched::feature::FixtureFeatureType;
use mlc_common::utils::{FormatEffectDuration, IntRange};
use crate::program_panel::EffectInvalidate;
use crate::utils::toaster::{Toaster, ToasterWriter};

#[derive(Debug, PartialEq, Copy, Clone)]
enum CreateTrackType {
    //Feature
    FeatureDimmer,
    FeatureColor,
    FeatureWhite,
    FeatureAmber,
    FeaturePanTilt,
    //Other
    FaderRaw,
    //Misc
    None,
}

#[component]
pub fn EffectTimeline() -> Element {
    let mut current_effect = use_context::<Signal<Option<Effect>>>();
    let effect_invalidator: Coroutine<EffectInvalidate> = use_coroutine_handle();

    let mut create_track_overlay = use_signal(|| false);
    let create_track_type = use_signal(|| CreateTrackType::None);

    let timeline_scale = use_signal(|| 5.0);

    if let Some(_effect) = current_effect() {
        rsx! {
            div {
                class: "effect-timeline",
                div {
                    class: "toolbar",
                    button {
                        onclick: move |_| {
                            create_track_overlay.set(true);
                        },
                        icons::Plus { width: "1.2rem", height: "1.2rem"},

                    },
                    button {
                        icons::Play { width: "1rem", height: "1rem"}
                    }
                },
                EffectTracks {
                    current_effect,
                    scale: timeline_scale,
                }

            },

            if create_track_overlay() {
                utils::Overlay {
                    title: "Create Effect Track",
                    class: "create-effect-track",
                    icon: rsx!{icons::TrainTrack {}},
                    onclose: move |_| {create_track_overlay.set(false);},
                    div {
                        class: "kind-list",
                        p {
                            class: "category",
                            "Feature Tracks"
                        },
                        CreateTrackOption {
                            create_track_type,
                            name: "Dimmer",
                            track_type: CreateTrackType::FeatureDimmer,
                        },
                        CreateTrackOption {
                            create_track_type,
                            name: "Color",
                            track_type: CreateTrackType::FeatureColor,
                        },
                        CreateTrackOption {
                            create_track_type,
                            name: "White",
                            track_type: CreateTrackType::FeatureWhite,
                        },
                        CreateTrackOption {
                            create_track_type,
                            name: "Amber",
                            track_type: CreateTrackType::FeatureAmber,
                        },
                        CreateTrackOption {
                            create_track_type,
                            name: "Pan/Tilt",
                            track_type: CreateTrackType::FeaturePanTilt,
                        },
                        p {
                            class: "category",
                            "Other"
                        },
                        CreateTrackOption {
                            create_track_type,
                            name: "Fader",
                            track_type: CreateTrackType::FaderRaw,
                        },
                    },
                    div {
                        class: "kind-options",
                        match create_track_type() {
                            CreateTrackType::FeatureDimmer => {
                                rsx!{
                                    CreateTrackDetailFeature {
                                        feature_type: FixtureFeatureType::Dimmer,
                                        onclose: move |t| {
                                            {
                                                let mut w = current_effect.write();
                                                if let Some(w) = &mut *w {
                                                    w.tracks.push(t);
                                                }
                                            }
                                            effect_invalidator.send(EffectInvalidate);
                                            create_track_overlay.set(false);
                                        }
                                    }
                                }
                            }
                            CreateTrackType::FeatureColor => {
                                rsx!{
                                    CreateTrackDetailFeature {
                                        feature_type: FixtureFeatureType::Rgb,
                                        onclose: move |t| {
                                            {
                                                let mut w = current_effect.write();
                                                if let Some(w) = &mut *w {
                                                    w.tracks.push(t);
                                                }
                                            }
                                            effect_invalidator.send(EffectInvalidate);
                                            create_track_overlay.set(false);
                                        }
                                    }
                                }
                            }
                            CreateTrackType::FeatureWhite => {
                                rsx!{
                                    CreateTrackDetailFeature {
                                        feature_type: FixtureFeatureType::White,
                                        onclose: move |t| {
                                            {
                                                let mut w = current_effect.write();
                                                if let Some(w) = &mut *w {
                                                    w.tracks.push(t);
                                                }
                                            }
                                            effect_invalidator.send(EffectInvalidate);
                                            create_track_overlay.set(false);
                                        }
                                    }
                                }
                            }
                            CreateTrackType::FeatureAmber => {
                                rsx!{
                                    CreateTrackDetailFeature {
                                        feature_type: FixtureFeatureType::Amber,
                                        onclose: move |t| {
                                            {
                                                let mut w = current_effect.write();
                                                if let Some(w) = &mut *w {
                                                    w.tracks.push(t);
                                                }
                                            }
                                            effect_invalidator.send(EffectInvalidate);
                                            create_track_overlay.set(false);
                                        }
                                    }
                                }
                            }
                            CreateTrackType::FeaturePanTilt => {
                                rsx!{
                                    CreateTrackDetailFeature {
                                        feature_type: FixtureFeatureType::PanTilt,
                                        onclose: move |t| {
                                            {
                                                let mut w = current_effect.write();
                                                if let Some(w) = &mut *w {
                                                    w.tracks.push(t);
                                                }
                                            }
                                            effect_invalidator.send(EffectInvalidate);
                                            create_track_overlay.set(false);
                                        }
                                    }
                                }
                            }
                            CreateTrackType::FaderRaw => {
                                rsx!{
                                    CreateTrackDetailFader {
                                        onclose: move |t| {
                                            {
                                                let mut w = current_effect.write();
                                                if let Some(w) = &mut *w {
                                                    w.tracks.push(t);
                                                }
                                            }
                                            effect_invalidator.send(EffectInvalidate);
                                            create_track_overlay.set(false);
                                        }
                                    }
                                }
                            }
                            CreateTrackType::None => {
                                rsx!{
                                    "Please select a Track type"
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "no-effect",
                p {
                    "No effect loaded!"
                }
            }
        }
    }
}

#[component]
fn CreateTrackOption(create_track_type: Signal<CreateTrackType>, name: String, track_type: CreateTrackType) -> Element {
    rsx! {
        p {
            class: "option",
            class: if create_track_type() == track_type {"sel"},
            onclick: move |_| {
                create_track_type.set(track_type);
            },
            {name}
        }
    }
}

#[component]
fn CreateTrackDetailFader(onclose: EventHandler<Track>) -> Element {
    let universes = use_resource(|| async {
        utils::fetch::<Vec<UniverseId>>("/data/universes").await.unwrap()
    });

    let mut toaster = use_context::<Signal<Toaster>>();

    let mut sel_universe = use_signal(|| "".to_string());
    let mut sel_address = use_signal(|| 0);

    rsx! {
        div {
            class: "property",
            p {
                "Universe"
            },
            select {
                onchange: move |e| {
                    sel_universe.set(e.value());
                },
                option {
                    value: "",
                    "-- Please select a Universe --"
                },
                match &*universes.read_unchecked() {
                    None => {rsx!()}
                    Some(us) => {
                        rsx! {
                            for universe in us {
                                option {
                                    value: universe.0 as i64,
                                    {format!("Universe: {}", universe.0)}
                                }
                            }
                        }
                    }
                }

            }
        },
        div {
            class: "property",
            p {
                "Address"
            },
            input {
                r#type: "number",
                min: 0,
                max: 511,
                value: sel_address().range(0, 511),
                oninput: move |e| {
                    let val = e.value().parse::<i64>().unwrap_or(0).range(0, 511);
                    sel_address.set(val);
                }
            }
        },
        button {
            class: "create-button",
            onclick: move |_| {
                let address = sel_address.peek().range(0, 511) as u16;
                let universe = sel_universe.peek().clone().parse::<u16>();
                if let Ok(u) = universe {
                    onclose.call(Track::FaderTrack(FaderTrack {
                        address: FaderAddress {
                            address: UniverseAddress::create(address).expect("Handled by range"),
                            universe: UniverseId(u)
                        },
                        values: vec![]
                    }));
                } else {
                    toaster.error("Invalid Universe", "Please select a Universe.");
                }
            },
            "Create Track"
        }
    }
}

#[component]
fn CreateTrackDetailFeature(onclose: EventHandler<Track>, feature_type: FixtureFeatureType) -> Element {
    let mut all_features = use_resource(move || async move {
        utils::fetch::<Vec<(FixtureId, String)>>(&format!("/data/all_with_feature/{}", &feature_type)).await.unwrap_or(vec![])
    });

    use_effect(use_reactive!(|feature_type| {
        let _ = feature_type;
        all_features.restart();
    }));

    let mut added_fixtures = use_signal(|| vec![]);
    let mut resolution = use_signal(|| 50);

    match (&*all_features.read_unchecked()).clone() {
        None => { rsx!("Loading available fixtures...") }
        Some(all) => {
            rsx! {
                div {
                    class: "property",
                    p {
                        "Track Resolution (ms)",
                    },
                    input {
                        r#type: "number",
                        value: resolution(),
                        min: 5,
                        oninput: move |e| {
                            let v = e.value().parse::<i64>().unwrap_or(50);
                            resolution.set(v);
                        }
                    }
                },
                div {
                    class: "property",
                    p {
                        "Select affected Fixtures:"
                    }
                },

                for id in all {
                    div {
                        class: "property",
                        p {
                            title: id.0.to_string(),
                            {id.1.clone()}
                        },
                        utils::Toggle {
                            value: added_fixtures().contains(&id.0),
                            onchange: move |v| {
                                let mut w = added_fixtures.write();
                                if v {
                                    if !w.contains(&id.0) {
                                        w.push(id.0);
                                    }
                                } else {
                                    let i = w.iter().position(|e| e == &id.0);
                                    if let Some(index) = i {
                                        w.remove(index);
                                    }
                                }
                            }
                        }
                    }
                }

                button {
                    class: "create-button",
                    onclick: move |_| {
                        let fixtures = added_fixtures();
                        onclose.call(Track::FeatureTrack(FeatureTrack{
                            resolution: Duration::milliseconds(resolution().max(5)),
                            feature: feature_type,
                            fixtures,
                            detail: FeatureTrackDetail::empty_from_feature_type(&feature_type),
                        }));
                    },
                    "Create Track"
                }
            }
        }
    }
}

#[component]
fn EffectTracks(current_effect: Signal<Option<Effect>>, scale: ReadOnlySignal<f32>) -> Element {
    let effect_invalidator: Coroutine<EffectInvalidate> = use_coroutine_handle();


    let effect = current_effect.map(|e| e.as_ref().expect("Should only be called with a valid effect!"));
    let duration_width = use_memo(move || {
        to_scaled_px(&current_effect().as_ref().unwrap().duration, scale())
    });

    let mut current_duration = use_signal(|| Duration::milliseconds(500));
    let current_duration_px = use_memo(move || {
        to_scaled_px(&current_duration(), scale())
    });


    let mut expanded = use_signal(|| HashSet::<usize>::new());

    rsx! {
        div {
            class: "track-container",
            div {
                class: "headers",
                div {
                    class: "header top",
                    {current_duration().effect_format()}
                }
                for (i, _track) in effect().tracks.iter().cloned().enumerate() {
                    div {
                        class: "header",
                        class: if expanded().contains(&i) {"expanded"},
                        div {
                            class: "expand-btn",
                            onclick: move |_| {
                                if !expanded.write().remove(&i) {
                                    expanded.write().insert(i);
                                }
                            },
                            match expanded().contains(&i) {
                                true => rsx!(icons::ArrowDown {}),
                                false => rsx!(icons::ArrowRight {})
                            }
                        },
                        "Track"
                    }
                }
            },
            div {
                class: "tracks",
                style: "--duration-width: {duration_width()}px;",
                div {
                    class: "track top",
                    onclick: move |e| {
                        current_duration.set(from_scaled_px(e.element_coordinates().x.max(0.0), scale()));
                    },
                    onmousemove: move |e| {
                        if e.held_buttons() == MouseButton::Primary {
                            current_duration.set(from_scaled_px(e.element_coordinates().x.max(0.0), scale()));
                        }
                    },
                    for i in 0..effect().duration.num_milliseconds() / 100 {
                        div {
                            class: "sec",
                            style: format!("--time-px: {}px", to_scaled_px_ms(i * 100, scale())),
                        }
                    }
                    div {
                        class: "time-marker",
                        style: "--duration-px: {current_duration_px()}px;",
                    }

                }
                for (i, track) in effect().tracks.iter().cloned().enumerate() {
                    div {
                        class: "track",
                        class: if expanded().contains(&i) {"expanded"},
                        div {
                            class: "time-marker",
                            style: "--duration-px: {current_duration_px()}px;",
                        }
                        match track {
                            Track::FaderTrack(track) => {
                                rsx! {
                                    FaderTrackBody {
                                        track,
                                        track_index: i,
                                        current_effect,
                                        invalidate: move |_| {
                                            effect_invalidator.send(EffectInvalidate);
                                        }
                                    }
                                }
                            },
                            Track::FeatureTrack(track) => {
                                rsx!{
                                    FeatureTrackBody {
                                        track,
                                        track_index: i,
                                        current_effect,
                                        invalidate: move |_| {
                                            effect_invalidator.send(EffectInvalidate);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FaderTrackBody(track: FaderTrack, current_effect: Signal<Option<Effect>>, track_index: usize, invalidate: EventHandler) -> Element {
    rsx! {
    }
}

#[component]
fn FeatureTrackBody(track: FeatureTrack, current_effect: Signal<Option<Effect>>, track_index: usize, invalidate: EventHandler) -> Element {
    rsx! {
    }
}

fn to_scaled_px(duration: &Duration, scale: f32) -> f64 {
    to_scaled_px_ms(duration.num_milliseconds(), scale)
}

fn to_scaled_px_ms(ms: i64, scale: f32) -> f64 {
    (ms as f64 / 10.0) * (scale as f64)
}

fn from_scaled_px(px: f64, scale: f32) -> Duration {
    Duration::milliseconds(((px / (scale as f64)) * 10.0) as i64)
}

fn with_track<F>(mut e: Signal<Option<Effect>>, track_index: usize, mut closure: F) where F: FnMut(&mut Track) {
    let mut r = e.write();
    let t = &mut r.as_mut().expect("Is only allowed to be called when a effect is loaded").tracks[track_index];
    closure(t);
}

fn with_fader_track<F>(e: Signal<Option<Effect>>, track_index: usize, mut closure: F) where F: FnMut(&mut FaderTrack) {
    with_track(e, track_index, |t| {
        match t {
            Track::FaderTrack(f) => { closure(f); }
            Track::FeatureTrack(_) => { log::error!("with_fader_track was called but track is a FeatureTrack!"); }
        }
    });
}

fn with_feature_track<F>(e: Signal<Option<Effect>>, track_index: usize, mut closure: F) where F: FnMut(&mut FeatureTrack) {
    with_track(e, track_index, |t| {
        match t {
            Track::FaderTrack(_) => { log::error!("with_feature_track was called but track is a FaderTrack!"); }
            Track::FeatureTrack(f) => { closure(f); }
        }
    });
}