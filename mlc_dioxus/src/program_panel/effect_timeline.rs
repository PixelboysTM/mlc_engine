use crate::configure_panel::Fader;
use crate::program_panel::key_editor::KeyEditor;
use crate::program_panel::EffectInvalidate;
use crate::utils::context_menu::ContextMenu;
use crate::utils::toaster::{Toaster, ToasterWriter};
use crate::{icons, utils};
use chrono::Duration;
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use mlc_common::effect::{
    D2RotationTrack, D3PercentTrack, Effect, FaderKey, FaderTrack, FeatureTrack,
    FeatureTrackDetail, Key, PercentTrack, RotationTrack, Track,
};
use mlc_common::fixture::FaderAddress;
use mlc_common::patched::feature::FixtureFeatureType;
use mlc_common::patched::{FixtureId, UniverseAddress, UniverseId};
use mlc_common::utils::bounds::{DynamicI64, Zero};
use mlc_common::utils::{BoundedValue, FormatEffectDuration};
use std::collections::HashSet;

use super::key_editor::DrawKeyWidget;

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

    let id = current_scope_id();
    use_effect(move || {
        let _ = current_effect();
        if let Some(i) = id {
            needs_update_any(i);
        }
    });

    let effect_invalidator: Coroutine<EffectInvalidate> = use_coroutine_handle();

    let mut create_track_overlay = use_signal(|| false);
    let create_track_type = use_signal(|| CreateTrackType::None);

    let timeline_scale = use_signal(|| 5.0);

    if let Some(_effect) = current_effect() {
        rsx! {
            div { class: "effect-timeline",
                div { class: "toolbar",
                    button {
                        onclick: move |_| {
                            create_track_overlay.set(true);
                        },
                        icons::Plus { width: "1.2rem", height: "1.2rem" }
                    }
                    button {
                        icons::Play { width: "1rem", height: "1rem" }
                    }
                    div {}
                    p { {format!("{:.1}x", timeline_scale())} }
                    utils::RangeSlider { value: timeline_scale, min: 0.5, max: 10.01, step: 0.1 }
                }
                EffectTracks { current_effect, scale: timeline_scale }
            }

            if create_track_overlay() {
                utils::Overlay {
                    title: "Create Effect Track",
                    class: "create-effect-track",
                    icon: rsx! {
                        icons::TrainTrack {}
                    },
                    onclose: move |_| {
                        create_track_overlay.set(false);
                    },
                    div { class: "kind-list",
                        p { class: "category", "Feature Tracks" }
                        CreateTrackOption {
                            create_track_type,
                            name: "Dimmer",
                            track_type: CreateTrackType::FeatureDimmer
                        }
                        CreateTrackOption {
                            create_track_type,
                            name: "Color",
                            track_type: CreateTrackType::FeatureColor
                        }
                        CreateTrackOption {
                            create_track_type,
                            name: "White",
                            track_type: CreateTrackType::FeatureWhite
                        }
                        CreateTrackOption {
                            create_track_type,
                            name: "Amber",
                            track_type: CreateTrackType::FeatureAmber
                        }
                        CreateTrackOption {
                            create_track_type,
                            name: "Pan/Tilt",
                            track_type: CreateTrackType::FeaturePanTilt
                        }
                        p { class: "category", "Other" }
                        CreateTrackOption { create_track_type, name: "Fader", track_type: CreateTrackType::FaderRaw }
                    }
                    div { class: "kind-options",
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
            div { class: "no-effect",
                p { "No effect loaded!" }
            }
        }
    }
}

#[component]
fn CreateTrackOption(
    create_track_type: Signal<CreateTrackType>,
    name: String,
    track_type: CreateTrackType,
) -> Element {
    rsx! {
        p {
            class: "option",
            class: if create_track_type() == track_type { "sel" },
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
        utils::fetch::<Vec<UniverseId>>("/data/universes")
            .await
            .unwrap()
    });

    let mut toaster = use_context::<Signal<Toaster>>();

    let mut sel_universe = use_signal(|| "".to_string());
    let mut sel_address = use_signal(|| 0);

    rsx! {
        div { class: "property",
            p { "Universe" }
            select {
                onchange: move |e| {
                    sel_universe.set(e.value());
                },
                option { value: "", "-- Please select a Universe --" }
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
        }
        div { class: "property",
            p { "Address" }
            input {
                r#type: "number",
                min: 0,
                max: 511,
                // value: sel_address().range(0, 511),
                value: BoundedValue::<_, Zero, DynamicI64<511>>::once(sel_address()),
                oninput: move |e| {
                    let val = BoundedValue::<
                        _,
                        Zero,
                        DynamicI64<511>,
                    >::once(e.value().parse().expect("Must be"));
                    sel_address.set(val);
                }
            }
        }
        button {
            class: "create-button",
            onclick: move |_| {
                let address = BoundedValue::<_, Zero, DynamicI64<511>>::once(*sel_address.peek())
                    as u16;
                let universe = sel_universe.peek().clone().parse::<u16>();
                if let Ok(u) = universe {
                    onclose
                        .call(
                            Track::FaderTrack(FaderTrack {
                                address: FaderAddress {
                                    address: UniverseAddress::create(address)
                                        .expect("Handled by range"),
                                    universe: UniverseId(u),
                                },
                                values: vec![],
                            }),
                        );
                } else {
                    toaster.error("Invalid Universe", "Please select a Universe.");
                }
            },
            "Create Track"
        }
    }
}

#[component]
fn CreateTrackDetailFeature(
    onclose: EventHandler<Track>,
    feature_type: FixtureFeatureType,
) -> Element {
    let mut all_features = use_resource(move || async move {
        utils::fetch::<Vec<(FixtureId, String)>>(&format!(
            "/data/all_with_feature/{}",
            &feature_type
        ))
        .await
        .unwrap_or(vec![])
    });

    use_effect(use_reactive!(|feature_type| {
        let _ = feature_type;
        all_features.restart();
    }));

    let mut added_fixtures = use_signal(Vec::new);
    let mut resolution = use_signal(|| 50);

    match (*all_features.read_unchecked()).clone() {
        None => {
            rsx!("Loading available fixtures...")
        }
        Some(all) => {
            rsx! {
                div { class: "property",
                    p { "Track Resolution (ms)" }
                    input {
                        r#type: "number",
                        value: resolution(),
                        min: 5,
                        oninput: move |e| {
                            let v = e.value().parse::<i64>().unwrap_or(50);
                            resolution.set(v);
                        }
                    }
                }
                div { class: "property",
                    p { "Select affected Fixtures:" }
                }

                for id in all {
                    div { class: "property",
                        p { title: id.0.to_string(), {id.1.clone()} }
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
                        onclose
                            .call(
                                Track::FeatureTrack(FeatureTrack {
                                    resolution: Duration::milliseconds(resolution().max(5)),
                                    feature: feature_type,
                                    fixtures,
                                    detail: FeatureTrackDetail::empty_from_feature_type(&feature_type),
                                }),
                            );
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

    let effect = current_effect.map(|e| {
        e.as_ref()
            .expect("Should only be called with a valid effect!")
    });
    let duration_width =
        use_memo(move || to_scaled_px(&current_effect().as_ref().unwrap().duration, scale()));

    let mut current_duration = use_signal(|| Duration::milliseconds(500));
    let current_duration_px = use_memo(move || to_scaled_px(&current_duration(), scale()));

    let mut track_context = use_signal(|| None);

    let mut expanded = use_signal(HashSet::<usize>::new);

    rsx! {
        if let Some(menu) = track_context() {
            utils::context_menu::ContextMenu {
                menu,
                onclose: move |_| {
                    track_context.set(None);
                }
            }
        }
        div { class: "track-container",
            div { class: "headers",
                div { class: "header top", {current_duration().effect_format()} }
                for (i , _track) in effect().tracks.iter().cloned().enumerate() {
                    div {
                        class: "header",
                        class: if expanded().contains(&i) { "expanded" },
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
                        }
                        {format!("Track #{}", i)}
                    }
                }
            }
            div {
                class: "tracks",
                style: "--duration-width: {duration_width()}px;",
                div {
                    class: "track top",
                    onclick: move |e| {
                        current_duration
                            .set(from_scaled_px(e.element_coordinates().x.max(0.0), scale()));
                    },
                    onmousemove: move |e| {
                        if e.held_buttons() == MouseButton::Primary {
                            current_duration
                                .set(from_scaled_px(e.element_coordinates().x.max(0.0), scale()));
                        }
                    },
                    for i in 0..(effect().duration.num_milliseconds() / 100 + 1) {
                        div {
                            class: "sec",
                            style: format!("--time-px: {}px", to_scaled_px_ms(i * 100, scale()))
                        }
                    }
                    div {
                        class: "time-marker",
                        style: "--duration-px: {current_duration_px()}px;"
                    }
                }
                for (i , track) in effect().tracks.iter().cloned().enumerate() {
                    div {
                        class: "track",
                        class: if expanded().contains(&i) { "expanded" },
                        oncontextmenu: move |e| {
                            if e.trigger_button() == Some(MouseButton::Secondary) {
                                let x_pos = e.element_coordinates().x;
                                track_context
                                    .set(
                                        Some(
                                            ContextMenu::new(
                                                    e.client_coordinates().x,
                                                    e.client_coordinates().y,
                                                )
                                                .add(
                                                    "Insert Keyframe here",
                                                    move |_| {
                                                        log::info!("Insert keyframe");
                                                        with_track(
                                                            current_effect,
                                                            i,
                                                            effect_invalidator,
                                                            move |t, d, _| {
                                                                let time = from_scaled_px(x_pos, scale());
                                                                match t {
                                                                    Track::FaderTrack(ft) => {
                                                                        if ft.values.is_empty() {
                                                                            ft.values
                                                                                .push(FaderKey {
                                                                                    value: 0,
                                                                                    start_time: Duration::milliseconds(0),
                                                                                });
                                                                            ft.values
                                                                                .push(FaderKey {
                                                                                    value: 0,
                                                                                    start_time: *d,
                                                                                });
                                                                        }
                                                                        ft.values
                                                                            .push(FaderKey {
                                                                                value: 0,
                                                                                start_time: time,
                                                                            });
                                                                    }
                                                                    Track::FeatureTrack(ft) => {
                                                                        if ft.is_empty() {
                                                                            ft.insert_default_key(Duration::milliseconds(0));
                                                                            ft.insert_default_key(*d);
                                                                        }
                                                                        ft.insert_default_key(time);
                                                                    }
                                                                }
                                                            },
                                                        );
                                                        true
                                                    },
                                                ),
                                        ),
                                    );
                            }
                            e.web_event().prevent_default();
                            e.stop_propagation();
                        },
                        div {
                            class: "time-marker",
                            style: "--duration-px: {current_duration_px()}px;"
                        }
                        match track {
                            Track::FaderTrack(track) => {
                                rsx! {
                                    FaderTrackBody {
                                        track,
                                        scale,
                                        track_index: i,
                                        current_effect,
                                        invalidate: effect_invalidator,
                                    }
                                }
                            },
                            Track::FeatureTrack(track) => {
                                rsx!{
                                    FeatureTrackBody {
                                        track,
                                        scale,
                                        track_index: i,
                                        current_effect,
                                        invalidate: effect_invalidator,
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
fn FaderTrackBody(
    track: FaderTrack,
    current_effect: Signal<Option<Effect>>,
    track_index: usize,
    invalidate: Coroutine<EffectInvalidate>,
    scale: ReadOnlySignal<f32>,
) -> Element {
    let mut key_edit = use_signal(|| None);

    rsx! {
        for (i , key) in track.values.into_iter().enumerate() {
            div {
                class: "key fader",
                title: key.start_time.effect_format(),
                style: format!(
                    "--kp-x: {}px; --k-vp: {}%;",
                    to_scaled_px(&key.start_time, *scale.read()),
                    (key.value as f32 / 255.0) * 100.0,
                ),
                onclick: move |e| {
                    log::info!("Context");
                    key_edit
                        .set(
                            Some((
                                key.clone(),
                                e.client_coordinates().x,
                                e.client_coordinates().y,
                                i,
                            )),
                        );
                },
                oncontextmenu: move |e| {
                    e.stop_propagation();
                },
                icons::DiamondFilled { width: "1rem", height: "1rem" }
            }
        }

        if let Some(key) = key_edit() {
            KeyEditor {
                px: key.1,
                py: key.2,
                onclose: move |_| {
                    key_edit.set(None);
                },
                Fader {
                    value: key.0.value,
                    id: "FDR".to_string(),
                    onchange: move |v| {
                        with_fader_track(
                            current_effect,
                            track_index,
                            invalidate,
                            move |t, _, _| {
                                t.values[key.3].value = v;
                            },
                        );
                    }
                }
            }
        }
    }
}

#[component]
fn FeatureTrackBody(
    track: FeatureTrack,
    current_effect: Signal<Option<Effect>>,
    track_index: usize,
    invalidate: Coroutine<EffectInvalidate>,
    scale: ReadOnlySignal<f32>,
) -> Element {
    rsx! {
        match track.detail {
            FeatureTrackDetail::SinglePercent(t) => rsx!{ {draw_generic_keys(
                &t.values,
            current_effect,
            track_index,
            invalidate,
            scale,
            |v| {
                let val = (v * 255.0) as u8;
                (val, val, val)
            },
            move |i, v| {
                with_percentage_track(current_effect, track_index, invalidate, |t| {
                    t.values[i].value = v;
                });
            },
        )}},
        FeatureTrackDetail::SingleRotation(t) => rsx!{ {draw_generic_keys(
            &t.values,
            current_effect,
            track_index,
            invalidate,
            scale,
            |v| {
                let val = ((v / 2.0 + 0.5) * 255.0) as u8;
                (val, val, val)
            },
            move |i, v| {
                with_rotation_track(current_effect, track_index, invalidate, |t| {
                    t.values[i].value = v;
                });
            },
        )}},
        FeatureTrackDetail::D3Percent(t) => rsx!{ {draw_generic_keys(
            &t.values,
            current_effect,
            track_index,
            invalidate,
            scale,
            |v| {
                let r = (v.0 * 255.0) as u8;
                let g = (v.1 * 255.0) as u8;
                let b = (v.2 * 255.0) as u8;
                (r, g, b)
            },
            move |i, v| {
                with_d3percent_track(current_effect, track_index, invalidate, |t| {
                    t.values[i].x = v.0;
                    t.values[i].y = v.1;
                    t.values[i].z = v.2;
                });
            },
        )}},
        FeatureTrackDetail::D2Rotation(t) => rsx!{ {draw_generic_keys(
            &t.values,
            current_effect,
            track_index,
            invalidate,
            scale,
            |v| {
                let u = (v.0 * 255.0) as u8;
                let v = (v.1 * 255.0) as u8;
                (u, v, 0)
            },
            move |i, v| {
                with_d2rotation_track(current_effect, track_index, invalidate, |t| {
                    t.values[i].x = v.0;
                    t.values[i].y = v.1;
                });
            },
        )}},
        }
    }
}

fn draw_generic_keys<K, F, F2>(
    keys: &[K],
    current_effect: Signal<Option<Effect>>,
    track_index: usize,
    invalidator: Coroutine<EffectInvalidate>,
    scale: ReadOnlySignal<f32>,
    color_fn: F,
    mut update_fn: F2,
) -> Element
where
    F: Fn(K::Value) -> (u8, u8, u8),
    F2: FnMut(usize, K::Value) + 'static,
    K: Key + DrawKeyWidget<K::Value> + Clone + 'static,
{
    let mut key_edit: Signal<Option<(K, f64, f64, usize)>> = use_signal(|| None);

    rsx! {
        if let Some(key) = key_edit() {
            KeyEditor {
                px: key.1,
                py: key.2,
                onclose: move |_| {
                    key_edit.set(None);
                },
                {key.0.draw_widget(move |v| update_fn(key.3, v))}
            }
        }

        for (i , key) in keys.iter().cloned().enumerate() {
            div {
                class: "key feature",
                title: key.time().effect_format(),
                style: format!(
                    "--kp-x: {}px; --k-vc: {};",
                    to_scaled_px(&key.time(), *scale.read()),
                    format!("rgb{:?}", color_fn(key.value())),
                ),
                onclick: move |e| {
                    log::info!("Context");
                    key_edit
                        .set(
                            Some((
                                key.clone(),
                                e.client_coordinates().x,
                                e.client_coordinates().y,
                                i,
                            )),
                        )
                },
                oncontextmenu: move |e| {
                    e.stop_propagation();
                },
                icons::DiamondFilled { width: "1rem", height: "1rem" }
            }
        }
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

fn with_track<F>(
    mut e: Signal<Option<Effect>>,
    track_index: usize,
    invalidator: Coroutine<EffectInvalidate>,
    mut closure: F,
) where
    F: FnMut(&mut Track, &Duration, &bool),
{
    let mut r = e.write();
    let e = r
        .as_mut()
        .expect("Is only allowed to be called when a effect is loaded");
    let t = &mut e.tracks[track_index];
    closure(t, &e.duration, &e.looping);
    invalidator.send(EffectInvalidate);
}

fn with_fader_track<F>(
    e: Signal<Option<Effect>>,
    track_index: usize,
    invalidator: Coroutine<EffectInvalidate>,
    mut closure: F,
) where
    F: FnMut(&mut FaderTrack, &Duration, &bool),
{
    with_track(e, track_index, invalidator, |t, d, l| match t {
        Track::FaderTrack(f) => {
            closure(f, d, l);
        }
        Track::FeatureTrack(_) => {
            log::error!("with_fader_track was called but track is a FeatureTrack!");
        }
    });
}

fn with_feature_track<F>(
    e: Signal<Option<Effect>>,
    track_index: usize,
    invalidator: Coroutine<EffectInvalidate>,
    mut closure: F,
) where
    F: FnMut(&mut FeatureTrack, &Duration, &bool),
{
    with_track(e, track_index, invalidator, |t, d, l| match t {
        Track::FaderTrack(_) => {
            log::error!("with_feature_track was called but track is a FaderTrack!");
        }
        Track::FeatureTrack(f) => {
            closure(f, d, l);
        }
    });
}

fn with_percentage_track<F>(
    e: Signal<Option<Effect>>,
    track_index: usize,
    invalidator: Coroutine<EffectInvalidate>,
    mut closure: F,
) where
    F: FnMut(&mut PercentTrack),
{
    with_feature_track(e, track_index, invalidator, move |t, _, _| {
        match &mut t.detail {
            FeatureTrackDetail::SinglePercent(t) => closure(t),
            FeatureTrackDetail::SingleRotation(_) => {
                log::error!("with_percentage_track was called but SingleRotation was supplied!")
            }
            FeatureTrackDetail::D3Percent(_) => {
                log::error!("with_percentage_track was called but D3Percent was supplied!")
            }
            FeatureTrackDetail::D2Rotation(_) => {
                log::error!("with_percentage_track was called but D2Rotation was supplied!")
            }
        }
    });
}

fn with_d3percent_track<F>(
    e: Signal<Option<Effect>>,
    track_index: usize,
    invalidator: Coroutine<EffectInvalidate>,
    mut closure: F,
) where
    F: FnMut(&mut D3PercentTrack),
{
    with_feature_track(e, track_index, invalidator, move |t, _, _| {
        match &mut t.detail {
            FeatureTrackDetail::D3Percent(t) => closure(t),
            FeatureTrackDetail::SingleRotation(_) => {
                log::error!("with_d3percent_track was called but SingleRotation was supplied!")
            }
            FeatureTrackDetail::SinglePercent(_) => {
                log::error!("with_d3percent_track was called but SinglePercent was supplied!")
            }
            FeatureTrackDetail::D2Rotation(_) => {
                log::error!("with_d3percent_track was called but D2Rotation was supplied!")
            }
        }
    });
}

fn with_rotation_track<F>(
    e: Signal<Option<Effect>>,
    track_index: usize,
    invalidator: Coroutine<EffectInvalidate>,
    mut closure: F,
) where
    F: FnMut(&mut RotationTrack),
{
    with_feature_track(e, track_index, invalidator, move |t, _, _| {
        match &mut t.detail {
            FeatureTrackDetail::SingleRotation(t) => closure(t),
            FeatureTrackDetail::D3Percent(_) => {
                log::error!("with_rotation_track was called but D3Rotation was supplied!")
            }
            FeatureTrackDetail::SinglePercent(_) => {
                log::error!("with_rotation_track was called but SinglePercent was supplied!")
            }
            FeatureTrackDetail::D2Rotation(_) => {
                log::error!("with_rotation_track was called but D2Rotation was supplied!")
            }
        }
    });
}

fn with_d2rotation_track<F>(
    e: Signal<Option<Effect>>,
    track_index: usize,
    invalidator: Coroutine<EffectInvalidate>,
    mut closure: F,
) where
    F: FnMut(&mut D2RotationTrack),
{
    with_feature_track(e, track_index, invalidator, move |t, _, _| {
        match &mut t.detail {
            FeatureTrackDetail::D2Rotation(t) => closure(t),
            FeatureTrackDetail::D3Percent(_) => {
                log::error!("with_d2rotation_track was called but D3Percent was supplied!")
            }
            FeatureTrackDetail::SinglePercent(_) => {
                log::error!("with_d2rotation_track was called but SinglePercent was supplied!")
            }
            FeatureTrackDetail::SingleRotation(_) => {
                log::error!("with_d2rotation_track was called but SingleRotation was supplied!")
            }
        }
    });
}
