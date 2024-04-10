use std::time::Duration;

use color_art::Color;
use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub use overlay::*;

use crate::icons;
use crate::utils::toaster::{Toaster, ToasterWriter};

pub mod toaster;
pub mod context_menu;
pub mod popover;
mod overlay;

pub async fn fetch<T>(url: &str) -> Result<T, gloo_net::Error>
    where
        T: DeserializeOwned,
{
    gloo_net::http::Request::get(url)
        .send()
        .await?
        .json::<T>()
        .await
}

pub async fn fetch_post<T, B>(url: &str, body: B) -> Result<T, gloo_net::Error>
    where
        T: DeserializeOwned,
        B: Serialize,
{
    gloo_net::http::Request::post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body)?)?
        .send()
        .await?
        .json::<T>()
        .await
}

pub async fn ws(url: &str) -> Result<WebSocket, String> {
    let host = gloo_utils::window().location().host().map_err(|e| format!("{e:?}"))?;
    WebSocket::open(&format!("ws://{}{}", host, url)).map_err(|e| format!("{e:?}"))
}

pub fn reload_window() -> Result<(), String> {
    gloo_utils::window().location().reload().map_err(|e| {
        format!("{e:?}")
    })
}

pub fn toast_reload(mut toaster: Signal<Toaster>) {
    let _ = reload_window().map_err(|e| {
        log::error!("{e}");
        toaster.error("Failed to reload window!", "See console for more detailed information!");
    });
}

#[component]
pub fn Loading() -> Element {
    let mut l = use_signal(|| false);

    use_future(move || {
        async move {
            if *l.peek() {
                log::warn!("In Spinner future is run twice");
                return;
            }
            async_std::task::sleep(Duration::from_millis(250)).await;
            l.set(true);
        }
    });

    if l() {
        rsx! {
            div {
                class: "loading-spinner",
                div {
                    class: "inner",
                }
            }
        }
    } else {
        rsx!("")
    }
}




#[component]
pub fn RgbWidget(
    initial: (f32, f32, f32),
    onchange: EventHandler<(f32, f32, f32)>,
) -> Element {
    let mut color = use_signal(||
        Color::from_rgb(initial.0 * 255.0, initial.1 * 255.0, initial.2 * 255.0).unwrap()
    );

    let hsv = use_memo(move || {
        let c = color();
        (c.hue(), c.hsv_saturation(), c.hsv_value())
    });

    let hue_col = use_memo(move || {
        color_art::Color::from_hsv(color().hue(), 1.0, 1.0).unwrap().hex()
    });

    let rgb = use_memo(move || {
        let c = color.read();
        (c.red(), c.green(), c.blue())
    });

    let to_color = |c: Color| {
        (
            c.red() as f32 / 255.0,
            c.green() as f32 / 255.0,
            c.blue() as f32 / 255.0,
        )
    };

    let mut hue_e = use_signal(|| None);
    let mut red_e = use_signal(|| None);
    let mut green_e = use_signal(|| None);
    let mut blue_e = use_signal(|| None);
    let mut sat_e = use_signal(|| None);

    rsx! {
        div {
            class: "rgb-widget",
            div {
                class: "sat",
                style: "--rgb-hue: {hue_col().to_string()}",
                tabindex: 0,
                onmounted: move |e| {
                    sat_e.set(Some(e.data));
                },
                onclick: move |e| {
                    async move {
                        let s = 1.0 - e.element_coordinates().x / sat_e().unwrap().get_client_rect().await.unwrap().size.width;
                        let v = 1.0 - e.element_coordinates().y / sat_e().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = color();
                        let v = Color::from_hsv(old.hsv_hue(), s.clamp(0.0, 1.0), v.clamp(0.0,1.0)).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onmousemove: move |e| {
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let s = 1.0 - e.element_coordinates().x / sat_e().unwrap().get_client_rect().await.unwrap().size.width;
                        let v = 1.0 - e.element_coordinates().y / sat_e().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = color();
                        let v = Color::from_hsv(old.hsv_hue(), s.clamp(0.0, 1.0), v.clamp(0.0,1.0)).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let mut changed = false;
                    let vl = color();
                    let (mut s, mut v) = (vl.hsv_saturation(), vl.hsv_value());
                    if e.key() == Key::ArrowUp {
                        v = (v + amount).clamp(0.0,1.0);
                        changed = true;
                    }
                    if e.key() == Key::ArrowDown {
                        v = (v - amount).clamp(0.0,1.0);
                        changed = true;
                    }
                    if e.key() == Key::ArrowLeft {
                        s = (s + amount).clamp(0.0,1.0);
                        changed = true;
                    }
                    if e.key() == Key::ArrowRight {
                        s = (s - amount).clamp(0.0,1.0);
                        changed = true;
                    }

                    if changed {
                        let v = Color::from_hsv(vl.hsv_hue(), s, v).expect("Is clamped");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({(1.0 - hsv().1) * 100.0}%, calc(100% - 0.5rem)); top: min({(1.0 - hsv().2) * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "hue",
                tabindex: 0,
                onmounted: move |e| {
                    hue_e.set(Some(e.data));
                },
                onmousedown: move |e| {
                    async move {
                        let h = e.element_coordinates().y / hue_e().unwrap().get_client_rect().await.unwrap().size.height * 360.0;
                        let old = color();
                        let v = Color::from_hsv(h.clamp(0.0, 360.0), old.hsv_saturation(), old.hsv_value()).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onmousemove: move |e| {
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let h = e.element_coordinates().y / hue_e().unwrap().get_client_rect().await.unwrap().size.height * 360.0;
                        let old = color();
                        let v = Color::from_hsv(h.clamp(0.0, 360.0), old.hsv_saturation(), old.hsv_value()).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.1} else {1.0};
                    let old = color();
                    if e.key() == Key::ArrowUp {
                        let h = (old.hsv_hue() - amount).clamp(0.0, 359.99);
                        let v = Color::from_hsv(h, old.hsv_saturation(), old.hsv_value()).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                    if e.key() == Key::ArrowDown {
                        let h = (old.hsv_hue() + amount).clamp(0.0, 359.99);
                        let v = Color::from_hsv(h, old.hsv_saturation(), old.hsv_value()).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                div {
                    class: "knob",
                    style: "top: min({hsv().0 / 360.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "val-r val",
                tabindex: 0,
                onmounted: move |e| {
                    red_e.set(Some(e.data));
                },
                onclick: move |e| {
                    async move {
                        let r = e.element_coordinates().x / red_e().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = color();
                        let v = Color::from_rgb(r.clamp(0.0, 255.0), old.green() as f64, old.blue() as f64).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onmousemove: move |e| {
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let r = e.element_coordinates().x / red_e().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = color();
                        let v = Color::from_rgb(r.clamp(0.0, 255.0), old.green() as f64, old.blue() as f64).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = color();
                    if e.key() == Key::ArrowLeft {
                        let r = ((old.red() as f32 / 255.0 - amount) * 255.0).clamp(0.0, 255.0) as u8;
                        let v = Color::from_rgb(r, old.green(), old.blue()).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                    if e.key() == Key::ArrowRight {
                        let r = ((old.red() as f32 / 255.0 + amount) * 255.0).clamp(0.0, 255.0) as u8;
                        let v = Color::from_rgb(r, old.green(), old.blue()).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({rgb().0 as f32 / 255.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "val-g val",
                tabindex: 0,
                onmounted: move |e| {
                    green_e.set(Some(e.data));
                },
                onclick: move |e| {
                    async move {
                        let g = e.element_coordinates().x / green_e().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = color();
                        let v = Color::from_rgb(old.red() as f64, g.clamp(0.0, 255.0), old.blue() as f64).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onmousemove: move |e| {
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let g = e.element_coordinates().x / green_e().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = color();
                        let v = Color::from_rgb(old.red() as f64, g.clamp(0.0, 255.0), old.blue() as f64).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = color();
                    if e.key() == Key::ArrowLeft {
                        let g = ((old.green() as f32 / 255.0 - amount) * 255.0).clamp(0.0, 255.0) as u8;
                        let v = Color::from_rgb(old.red(), g, old.blue()).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                    if e.key() == Key::ArrowRight {
                        let g = ((old.green() as f32 / 255.0 + amount) * 255.0).clamp(0.0, 255.0) as u8;
                        let v = Color::from_rgb(old.red(), g, old.blue()).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({rgb().1 as f32 / 255.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "val-b val",
                tabindex: 0,
                onmounted: move |e| {
                    blue_e.set(Some(e.data));
                },
                onclick: move |e| {
                    async move {
                        let b = e.element_coordinates().x / blue_e().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = color();
                        let v = Color::from_rgb(old.red() as f64, old.green() as f64, b.clamp(0.0, 255.0)).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onmousemove: move |e| {
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let b = e.element_coordinates().x / blue_e().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = color();
                        let v = Color::from_rgb(old.red() as f64, old.green() as f64, b.clamp(0.0, 255.0)).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = color();
                    if e.key() == Key::ArrowLeft {
                        let b = ((old.blue() as f32 / 255.0 - amount) * 255.0).clamp(0.0, 255.0) as u8;
                        let v = Color::from_rgb(old.red(), old.green(), b).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                    if e.key() == Key::ArrowRight {
                        let b = ((old.blue() as f32 / 255.0 + amount) * 255.0).clamp(0.0, 255.0) as u8;
                        let v = Color::from_rgb(old.red(), old.green(), b).expect("We have a problem!");
                        color.set(v);
                        onchange.call(to_color(v));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({rgb().2 as f32 / 255.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            }
        }
    }
}

#[component]
pub fn PanTiltWidget(
    initial: (f32, f32),
    onchange: EventHandler<(f32, f32)>,
) -> Element {
    let mut pt = use_signal(|| initial);

    let pan = use_memo(move || pt().0);

    let tilt = use_memo(move || pt().1);


    let mut pan_e = use_signal(|| None);
    let mut tilt_e = use_signal(|| None);
    let mut zone_e = use_signal(|| None);

    rsx! {
        div {
           class: "pan-tilt-widget",
            div {
                class: "d2-zone",
                style: "--line-x: min({tilt() * 100.0}%, 100%); --line-y: min({(1.0 - pan()) * 100.0}%, 100%);",
                tabindex: 0,
                onmounted: move |e| {
                    zone_e.set(Some(e.data));
                },
                onclick: move |e| {
                    async move {
                        let p = e.element_coordinates().y / zone_e().unwrap().get_client_rect().await.unwrap().size.height;
                        let t = e.element_coordinates().x / zone_e().unwrap().get_client_rect().await.unwrap().size.width;
                        let v = (p.clamp(0.0,1.0) as f32, t.clamp(0.0,1.0) as f32);
                        pt.set(v);
                        onchange.call(v);
                    }
                },
                onmousemove: move |e| {
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let p = e.element_coordinates().y / zone_e().unwrap().get_client_rect().await.unwrap().size.height;
                        let t = e.element_coordinates().x / zone_e().unwrap().get_client_rect().await.unwrap().size.width;
                        let v = (p.clamp(0.0,1.0) as f32, t.clamp(0.0,1.0) as f32);
                        pt.set(v);
                        onchange.call(v);
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let mut changed = false;
                    let mut v = pt();
                    if e.key() == Key::ArrowUp {
                        v.0 = (v.0 - amount).clamp(0.0,1.0);
                        changed = true;
                    }
                    if e.key() == Key::ArrowDown {
                        v.0 = (v.0 + amount).clamp(0.0,1.0);
                        changed = true;
                    }
                    if e.key() == Key::ArrowLeft {
                        v.1 = (v.1 - amount).clamp(0.0,1.0);
                        changed = true;
                    }
                    if e.key() == Key::ArrowRight {
                        v.1 = (v.1 + amount).clamp(0.0,1.0);
                        changed = true;
                    }

                    if changed {
                        pt.set(v);
                        onchange.call(v);
                    }
                },
            },
            div {
                class: "val-pan",
                tabindex: 0,
                onmounted: move |e| {
                    pan_e.set(Some(e.data));
                },
                onclick: move |e| {
                    async move {
                        let p = e.element_coordinates().y / pan_e().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = pt();
                        let v = (p.clamp(0.0,1.0) as f32, old.1);
                        pt.set(v);
                        onchange.call(v);
                    }
                },
                onmousemove: move |e| {
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let p = e.element_coordinates().y / pan_e().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = pt();
                        let v = (p.clamp(0.0,1.0) as f32, old.1);
                        pt.set(v);
                        onchange.call(v);
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = pt();
                    if e.key() == Key::ArrowUp {
                        let v = ((old.0 - amount).clamp(0.0,1.0) as f32, old.1);
                        pt.set(v);
                        onchange.call(v);
                    }
                    if e.key() == Key::ArrowDown {
                        let v = ((old.0 + amount).clamp(0.0,1.0) as f32, old.1);
                        pt.set(v);
                        onchange.call(v);
                    }
                },
                div {
                    class: "knob",
                    style: "top: min({pan() * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            }

            div {
                class: "val-tilt",
                tabindex: 0,
                onmounted: move |e| {
                    tilt_e.set(Some(e.data));
                },
                onclick: move |e| {
                    async move {
                        let t = e.element_coordinates().x / tilt_e().unwrap().get_client_rect().await.unwrap().size.width;
                        let old = pt();
                        let v = (old.0, t.clamp(0.0,1.0) as f32);
                        pt.set(v);
                        onchange.call(v);
                    }
                },
                onmousemove: move |e| {
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let t = e.element_coordinates().x / tilt_e().unwrap().get_client_rect().await.unwrap().size.width;
                        let old = pt();
                        let v = (old.0, t.clamp(0.0,1.0) as f32);
                        pt.set(v);
                        onchange.call(v);
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = pt();
                    if e.key() == Key::ArrowLeft {
                        let v = (old.0, (old.1 - amount).clamp(0.0,1.0));
                        pt.set(v);
                        onchange.call(v);
                    }
                    if e.key() == Key::ArrowRight {
                        let v = (old.0, (old.1 + amount).clamp(0.0,1.0));
                        pt.set(v);
                        onchange.call(v);
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({tilt() * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },

            div {
              class: "cross-btn",
                icons::Plus{
                    width: "100%",
                    height: "100%",
                }
            }
        }
    }
}

#[component]
pub fn Slider(initial: f32, onchange: EventHandler<f32>) -> Element {
    let mut val = use_signal(|| 1.0 - initial);

    let mut size_e = use_signal(|| None);

    rsx! {
        div {
            class: "slider-widget",
            style: "--line-x: {val() * 100.0}%",
            tabindex: 0,
            onmounted: move |e| {
                size_e.set(Some(e.data));
            },
            onmousedown: move |e| {
                async move {
                    if e.held_buttons() == MouseButton::Primary{
                        let h = e.element_coordinates().y / size_e().expect("Not mounted?").get_client_rect().await.unwrap().size.height;
                        let v = h.clamp(0.0,1.0) as f32;
                        val.set(v);
                        onchange.call(1.0 - v);
                    }
                }
            },
            onmousemove: move |e| {
                async move {
                    if e.held_buttons() == MouseButton::Primary{
                        let h = e.element_coordinates().y / size_e().expect("Not mounted?").get_client_rect().await.unwrap().size.height;
                        let v = h.clamp(0.0,1.0) as f32;
                        val.set(v);
                        onchange.call(1.0 - v);
                    }
                }
            },
            ontouchmove: move |e| {
                async move {
                    if !e.target_touches().is_empty(){
                        let rect = size_e().expect("Not mounted?").get_client_rect().await.unwrap();
                        let h = (e.target_touches()[0].client_coordinates().y - rect.origin.y ) / rect.size.height;
                        let v = h.clamp(0.0,1.0) as f32;
                        val.set(v);
                        onchange.call(1.0 - v);
                    }
                }
            },
            onkeydown: move |e| {
                let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                if e.key() == Key::ArrowUp {
                    let v = (val() - amount).clamp(0.0,1.0);
                    val.set(v);
                    onchange.call(1.0 - v);
                }
                if e.key() == Key::ArrowDown {
                    let v = (val() + amount).clamp(0.0,1.0);
                    val.set(v);
                    onchange.call(1.0 - v);
                }
            },
            div {
                class: "knob",
            }
        }
    }
}


#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum CheckboxState {
    #[default]
    Unchecked,
    Partly,
    Checked,
}

impl CheckboxState {
    pub fn toggle(&self) -> Self {
        match self {
            CheckboxState::Unchecked => Self::Checked,
            CheckboxState::Partly => Self::Checked,
            CheckboxState::Checked => Self::Unchecked,
        }
    }
}

impl From<bool> for CheckboxState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Checked,
            false => Self::Unchecked,
        }
    }
}

impl From<CheckboxState> for bool {
    fn from(value: CheckboxState) -> Self {
        match value {
            CheckboxState::Unchecked => false,
            CheckboxState::Partly => false,
            CheckboxState::Checked => true,
        }
    }
}

#[component]
pub fn Checkbox(init: CheckboxState, onchange: EventHandler<CheckboxState>) -> Element {
    let mut state = use_signal(|| init);
    rsx! {
        div {
            class: "checkbox-comp",
            onclick: move |e| {
                if e.trigger_button() == Some(MouseButton::Primary) {
                    let new_state = state().toggle();
                    state.set(new_state);
                    onchange.call(new_state);
                }
            },
            match state() {
                CheckboxState::Unchecked => {rsx!("")}
                CheckboxState::Partly => {rsx!(icons::Minus{
                    width: "1em",
                    height: "1em"
                })}
                CheckboxState::Checked => {rsx!(icons::Check{
                    width: "1em",
                    height: "1em"
                })}
            }
        }
    }
}

#[component]
pub fn Toggle(value: bool, onchange: Option<EventHandler<bool>>) -> Element {
    let mut signal_value = use_signal(|| value);
    use_effect(use_reactive!(|value| { signal_value.set(value)}));

    rsx! {
        div {
            class: "toggle-ele",
            onclick: move |_| {
                signal_value.toggle();
                if let Some(h) = onchange.as_ref() {
                    h.call(*signal_value.peek());
                }
            },
            div {
                class: "knob",
                class: if signal_value() {"activated"},
                style: "pointer-events: none;",
            }
        }
    }
}

pub trait ToWebSocketMessage {
    fn to_msg(self) -> Result<Message, serde_json::error::Error>;
}

impl<D: Serialize> ToWebSocketMessage for D {
    fn to_msg(self) -> Result<Message, serde_json::error::Error> {
        let json = serde_json::to_string(&self)?;
        Ok(Message::Text(json))
    }
}

#[component]
pub fn RangeSlider(value: Signal<f32>, min: f32, max: f32, step: f32) -> Element {
    rsx! {
        div {
            class: "range-slider-container",
            input {
                r#type: "range",
                min: min as f64,
                max: max as f64,
                step: step as f64,
                value: value() as f64,
                onchange: move |e| {
                    let val = e.value().parse::<f32>().unwrap_or(min);
                    value.set(val);
                }
            }
        }
    }
}