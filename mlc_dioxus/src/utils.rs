use std::time::Duration;

use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use gloo_net::websocket::futures::WebSocket;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::icons;

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

pub fn ws(url: &str) -> Result<WebSocket, gloo_utils::errors::JsError> {
    WebSocket::open(url)
}

#[component]
pub fn Loading(cx: Scope) -> Element {
    let l = use_state(cx, || false);

    use_on_create(cx, || {
        to_owned![l];
        async move {
            async_std::task::sleep(Duration::from_millis(250)).await;
            l.set(true);
        }
    });

    if *l.get() {
        cx.render(rsx! {
            div {
                class: "loading-spinner",
                div {
                    class: "inner",
                }
            }
        })
    } else {
        cx.render(rsx!(""))
    }
}

#[derive(Props)]
pub struct OverlayProps<'a> {
    #[props(into)]
    title: String,
    #[props(into)]
    class: String,
    icon: Element<'a>,
    onclose: EventHandler<'a, ()>,
    children: Element<'a>,
}

#[component]
pub fn Overlay<'a>(cx: Scope<'a, OverlayProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "overlay",
            onclick: move |_| {
              cx.props.onclose.call(());
            },
            div {
                class: "overlay-content {cx.props.class}",
                onclick: move |e| {
                    e.stop_propagation();
                },

                div {
                    class: "header",
                    div {
                        class: "icon-holder",
                        &cx.props.icon
                    },
                    h3 {
                        class: "title",
                        {cx.props.title.clone()}
                    },
                    button {
                        class: "icon close-btn",
                        onclick: move |_| {
                            cx.props.onclose.call(());
                        },
                        icons::X {
                            width: "2.5rem",
                            height: "2.5rem"
                        }
                    }
                },
                div {
                    class: "overlay-body",
                    &cx.props.children
                }
            },
        }
    })
}

#[component]
pub fn RgbWidget<'a>(
    cx: Scope<'a>,
    initial: (f32, f32, f32),
    onchange: EventHandler<'a, (f32, f32, f32)>,
) -> Element<'a> {
    // let initial: (f32, f32, f32) = (1.0, 1.0, 1.0);
    // let i = color_art::Color::from_rgb(initial.0 * 255.0, initial.1 * 255.0, initial.2 * 255.0).unwrap();
    let color = use_state(cx, || {
        color_art::Color::from_rgb(initial.0 * 255.0, initial.1 * 255.0, initial.2 * 255.0).unwrap()
    });

    let hsv = use_memo(cx, (color, ), |(c, )| {
        (c.hue(), c.hsv_saturation(), c.hsv_value())
    });

    let hue_col = use_memo(cx, (color, ), |(c, )| {
        color_art::Color::from_hsv(c.hue(), 1.0, 1.0).unwrap().hex()
    });

    let rgb = use_memo(cx, (color, ), |(c, )| (c.red(), c.green(), c.blue()));

    use_effect(cx, (color, ), |(c, )| {
        onchange.call((
            c.red() as f32 / 255.0,
            c.green() as f32 / 255.0,
            c.blue() as f32 / 255.0,
        ));
        async move {}
    });

    let hue_e = use_state(cx, || None);
    let red_e = use_state(cx, || None);
    let green_e = use_state(cx, || None);
    let blue_e = use_state(cx, || None);
    let sat_e = use_state(cx, || None);

    cx.render(rsx! {
        div {
            class: "rgb-widget",
            div {
                class: "sat",
                style: "--rgb-hue: {hue_col.to_string()}",
                tabindex: 0,
                onmounted: move |e| {
                    sat_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![sat_e, color];
                    async move {
                        let s = 1.0 - e.element_coordinates().x / sat_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width;
                        let v = 1.0 - e.element_coordinates().y / sat_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = *color.get();
                        color.set(color_art::Color::from_hsv(old.hsv_hue(), s.clamp(0.0, 1.0), v.clamp(0.0,1.0)).expect("We have a problem!"));
                    }
                },
                onmousemove: move |e| {
                    to_owned![sat_e, color];
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let s = 1.0 - e.element_coordinates().x / sat_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width;
                        let v = 1.0 - e.element_coordinates().y / sat_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = *color.get();
                        color.set(color_art::Color::from_hsv(old.hsv_hue(), s.clamp(0.0,1.0), v.clamp(0.0,1.0)).expect("We have a problem!"));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let mut changed = false;
                    let vl = *color.get();
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
                        color.set(color_art::Color::from_hsv(vl.hsv_hue(), s, v).expect("Is clamped"));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({(1.0 - hsv.1) * 100.0}%, calc(100% - 0.5rem)); top: min({(1.0 - hsv.2) * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "hue",
                tabindex: 0,
                onmounted: move |e| {
                    hue_e.set(Some(e.data));
                },
                onmousedown: move |e| {
                    to_owned![hue_e, color];
                    async move {
                        let h = e.element_coordinates().y / hue_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height * 360.0;
                        let old = *color.get();
                        color.set(color_art::Color::from_hsv(h.clamp(0.0, 360.0), old.hsv_saturation(), old.hsv_value()).expect("We have a problem!"));
                    }
                },
                onmousemove: move |e| {
                    to_owned![hue_e, color];
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let h = e.element_coordinates().y / hue_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height * 360.0;
                        let old = *color.get();
                        color.set(color_art::Color::from_hsv(h.clamp(0.0, 360.0), old.hsv_saturation(), old.hsv_value()).expect("We have a problem!"));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.1} else {1.0};
                    let old = *color.get();
                    if e.key() == Key::ArrowUp {
                        let h = (old.hsv_hue() - amount).clamp(0.0, 359.99);
                        color.set(color_art::Color::from_hsv(h, old.hsv_saturation(), old.hsv_value()).expect("We have a problem!"));
                    }
                    if e.key() == Key::ArrowDown {
                        let h = (old.hsv_hue() + amount).clamp(0.0, 359.99);
                        color.set(color_art::Color::from_hsv(h, old.hsv_saturation(), old.hsv_value()).expect("We have a problem!"));
                    }
                },
                div {
                    class: "knob",
                    style: "top: min({hsv.0 / 360.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "val-r val",
                tabindex: 0,
                onmounted: move |e| {
                    red_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![red_e, color];
                    async move {
                        let r = e.element_coordinates().x / red_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = *color.get();
                        color.set(color_art::Color::from_rgb(r.clamp(0.0, 255.0), old.green() as f64, old.blue() as f64).expect("We have a problem!"));
                    }
                },
                onmousemove: move |e| {
                    to_owned![red_e, color];
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let r = e.element_coordinates().x / red_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = *color.get();
                        color.set(color_art::Color::from_rgb(r.clamp(0.0, 255.0), old.green() as f64, old.blue() as f64).expect("We have a problem!"));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = *color.get();
                    if e.key() == Key::ArrowLeft {
                        let r = ((old.red() as f32 / 255.0 - amount) * 255.0).clamp(0.0, 255.0) as u8;
                        color.set(color_art::Color::from_rgb(r, old.green(), old.blue()).expect("We have a problem!"));
                    }
                    if e.key() == Key::ArrowRight {
                        let r = ((old.red() as f32 / 255.0 + amount) * 255.0).clamp(0.0, 255.0) as u8;
                        color.set(color_art::Color::from_rgb(r, old.green(), old.blue()).expect("We have a problem!"));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({rgb.0 as f32 / 255.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "val-g val",
                tabindex: 0,
                onmounted: move |e| {
                    green_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![green_e, color];
                    async move {
                        let g = e.element_coordinates().x / green_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = *color.get();
                        color.set(color_art::Color::from_rgb(old.red() as f64, g.clamp(0.0, 255.0), old.blue() as f64).expect("We have a problem!"));
                    }
                },
                onmousemove: move |e| {
                    to_owned![green_e, color];
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let g = e.element_coordinates().x / green_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = *color.get();
                        color.set(color_art::Color::from_rgb(old.red() as f64, g.clamp(0.0, 255.0), old.blue() as f64).expect("We have a problem!"));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = *color.get();
                    if e.key() == Key::ArrowLeft {
                        let g = ((old.green() as f32 / 255.0 - amount) * 255.0).clamp(0.0, 255.0) as u8;
                        color.set(color_art::Color::from_rgb(old.red(), g, old.blue()).expect("We have a problem!"));
                    }
                    if e.key() == Key::ArrowRight {
                        let g = ((old.green() as f32 / 255.0 + amount) * 255.0).clamp(0.0, 255.0) as u8;
                        color.set(color_art::Color::from_rgb(old.red(), g, old.blue()).expect("We have a problem!"));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({rgb.1 as f32 / 255.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "val-b val",
                tabindex: 0,
                onmounted: move |e| {
                    blue_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![blue_e, color];
                    async move {
                        let b = e.element_coordinates().x / blue_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = *color.get();
                        color.set(color_art::Color::from_rgb(old.red() as f64, old.green() as f64, b.clamp(0.0, 255.0)).expect("We have a problem!"));
                    }
                },
                onmousemove: move |e| {
                    to_owned![blue_e, color];
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let b = e.element_coordinates().x / blue_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = *color.get();
                        color.set(color_art::Color::from_rgb(old.red() as f64, old.green() as f64, b.clamp(0.0, 255.0)).expect("We have a problem!"));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = *color.get();
                    if e.key() == Key::ArrowLeft {
                        let b = ((old.blue() as f32 / 255.0 - amount) * 255.0).clamp(0.0, 255.0) as u8;
                        color.set(color_art::Color::from_rgb(old.red(), old.green(), b).expect("We have a problem!"));
                    }
                    if e.key() == Key::ArrowRight {
                        let b = ((old.blue() as f32 / 255.0 + amount) * 255.0).clamp(0.0, 255.0) as u8;
                        color.set(color_art::Color::from_rgb(old.red(), old.green(), b).expect("We have a problem!"));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({rgb.2 as f32 / 255.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            }
        }
    })
}

#[component]
pub fn PanTiltWidget<'a>(
    cx: Scope<'a>,
    initial: (f32, f32),
    onchange: EventHandler<'a, (f32, f32)>,
) -> Element<'a> {
    let pt = use_state(cx, || *initial);

    let pan = use_memo(cx, (pt, ), |(p, )| p.0);

    let tilt = use_memo(cx, (pt, ), |(t, )| t.1);

    use_effect(cx, (pt, ), |(pt, )| {
        let pt = *pt.get();
        onchange.call(pt);
        async move {}
    });

    let pan_e = use_state(cx, || None);
    let tilt_e = use_state(cx, || None);
    let zone_e = use_state(cx, || None);

    cx.render(rsx! {
        div {
           class: "pan-tilt-widget",
            div {
                class: "d2-zone",
                style: "--line-x: min({*tilt * 100.0}%, 100%); --line-y: min({(1.0 - *pan) * 100.0}%, 100%);",
                tabindex: 0,
                onmounted: move |e| {
                    zone_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![zone_e, pt];
                    async move {
                        let p = e.element_coordinates().y / zone_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height;
                        let t = e.element_coordinates().x / zone_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width;
                        pt.set((p.clamp(0.0,1.0) as f32, t.clamp(0.0,1.0) as f32));
                    }
                },
                onmousemove: move |e| {
                    to_owned![zone_e, pt];
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let p = e.element_coordinates().y / zone_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height;
                        let t = e.element_coordinates().x / zone_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width;
                        pt.set((p.clamp(0.0,1.0) as f32, t.clamp(0.0,1.0) as f32));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let mut changed = false;
                    let mut v = *pt.get();
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
                    to_owned![pan_e, pt];
                    async move {
                        let p = e.element_coordinates().y / pan_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = *pt.get();
                        pt.set((p.clamp(0.0,1.0) as f32, old.1));
                    }
                },
                onmousemove: move |e| {
                    to_owned![pan_e, pt];
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let p = e.element_coordinates().y / pan_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = *pt.get();
                        pt.set((p.clamp(0.0,1.0) as f32, old.1));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = *pt.get();
                    if e.key() == Key::ArrowUp {
                        pt.set(((old.0 - amount).clamp(0.0,1.0), old.1));
                    }
                    if e.key() == Key::ArrowDown {
                        pt.set(((old.0 + amount).clamp(0.0,1.0), old.1));
                    }
                },
                div {
                    class: "knob",
                    style: "top: min({*pan * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            }

            div {
                class: "val-tilt",
                tabindex: 0,
                onmounted: move |e| {
                    tilt_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![tilt_e, pt];
                    async move {
                        let t = e.element_coordinates().x / tilt_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width;
                        let old = *pt.get();
                        pt.set((old.0, t.clamp(0.0,1.0) as f32));
                    }
                },
                onmousemove: move |e| {
                    to_owned![tilt_e, pt];
                    async move {
                        if !e.held_buttons().contains(MouseButton::Primary) {
                            return;
                        }
                        let t = e.element_coordinates().x / tilt_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width;
                        let old = *pt.get();
                        pt.set((old.0, t.clamp(0.0,1.0) as f32));
                    }
                },
                onkeydown: move |e| {
                    let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                    let old = *pt.get();
                    if e.key() == Key::ArrowLeft {
                        pt.set((old.0, (old.1 - amount).clamp(0.0,1.0)));
                    }
                    if e.key() == Key::ArrowRight {
                        pt.set((old.0, (old.1 + amount).clamp(0.0,1.0)));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({*tilt * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
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
    })
}

#[component]
pub fn Slider<'a>(cx: Scope<'a>, initial: f32, onchange: EventHandler<'a, f32>) -> Element<'a> {
    let val = use_state(cx, || 1.0 - *initial);

    use_effect(cx, (val, ), |(v, )| {
        onchange.call(1.0 - *v.get());
        async {}
    });

    let size_e = use_state(cx, || None);

    cx.render(rsx! {
        div {
            class: "slider-widget",
            style: "--line-x: {*val.get() * 100.0}%",
            tabindex: 0,
            onmounted: move |e| {
                size_e.set(Some(e.data));
            },
            onmousedown: move |e| {
                to_owned![size_e, val];
                async move {
                    if e.held_buttons() == MouseButton::Primary{
                        let h = e.element_coordinates().y / size_e.get().as_ref().expect("Not mounted?").get_client_rect().await.unwrap().size.height;
                        val.set(h.clamp(0.0,1.0) as f32);
                    }
                }
            },
            onmousemove: move |e| {
                to_owned![size_e, val];
                async move {
                    if e.held_buttons() == MouseButton::Primary{
                        let h = e.element_coordinates().y / size_e.get().as_ref().expect("Not mounted?").get_client_rect().await.unwrap().size.height;
                        val.set(h.clamp(0.0,1.0) as f32);
                    }
                }
            },
            onkeydown: move |e| {
                let amount = if e.modifiers() == Modifiers::CONTROL {0.001} else {0.01};
                if e.key() == Key::ArrowUp {
                    val.set((*val.get() - amount).clamp(0.0,1.0))
                }
                if e.key() == Key::ArrowDown {
                    val.set((*val.get() + amount).clamp(0.0,1.0))
                }
            },
            div {
                class: "knob",
            }
        }
    })
}