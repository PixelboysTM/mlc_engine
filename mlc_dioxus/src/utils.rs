use std::time::Duration;
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
pub fn RgbWidget<'a>(cx: Scope<'a>, initial: (f32, f32, f32), onchange: EventHandler<'a, (f32, f32, f32)>) -> Element<'a> {
    // let initial: (f32, f32, f32) = (1.0, 1.0, 1.0);
    // let i = color_art::Color::from_rgb(initial.0 * 255.0, initial.1 * 255.0, initial.2 * 255.0).unwrap();
    let color = use_state(cx, || color_art::Color::from_rgb(initial.0 * 255.0, initial.1 * 255.0, initial.2 * 255.0).unwrap());

    let hsv = use_memo(cx, (color, ), |(c, )| {
        (c.hue(), c.hsv_saturation(), c.hsv_value())
    });

    let hue_col = use_memo(cx, (color, ), |(c, )| {
        color_art::Color::from_hsv(c.hue(), 1.0, 1.0).unwrap().hex()
    });

    let rgb = use_memo(cx, (color, ), |c, | {
        (color.red(), color.green(), color.blue())
    });

    let e = use_effect(cx, (color, ), |(c, )| {
        let c = c.get().clone();
        onchange.call((c.red() as f32 / 255.0, c.green() as f32 / 255.0, c.blue() as f32 / 255.0));
        async move {}
    });

    let mut hue_e = use_state(cx, || None);
    let mut red_e = use_state(cx, || None);
    let mut green_e = use_state(cx, || None);
    let mut blue_e = use_state(cx, || None);
    let mut sat_e = use_state(cx, || None);

    cx.render(rsx! {
        div {
            class: "rgb-widget",
            div {
                class: "sat",
                style: "--rgb-hue: {hue_col.to_string()}",
                onmounted: move |e| {
                    sat_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![sat_e, color];
                    async move {
                        let s = 1.0 - e.element_coordinates().x / sat_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width;
                        let v = 1.0 - e.element_coordinates().y / sat_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = color.get().clone();
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
                        let old = color.get().clone();
                        color.set(color_art::Color::from_hsv(old.hsv_hue(), s.clamp(0.0,1.0), v.clamp(0.0,1.0)).expect("We have a problem!"));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({(1.0 - hsv.1) * 100.0}%, calc(100% - 0.5rem)); top: min({(1.0 - hsv.2) * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "hue",
                onmounted: move |e| {
                    hue_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![hue_e, color];
                    async move {
                        let h = e.element_coordinates().y / hue_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height * 360.0;
                        let old = color.get().clone();
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
                        let old = color.get().clone();
                        color.set(color_art::Color::from_hsv(h.clamp(0.0, 360.0), old.hsv_saturation(), old.hsv_value()).expect("We have a problem!"));
                    }
                },
                div {
                    class: "knob",
                    style: "top: min({hsv.0 / 360.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "val-r val",
                onmounted: move |e| {
                    red_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![red_e, color];
                    async move {
                        let r = e.element_coordinates().x / red_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = color.get().clone();
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
                        let old = color.get().clone();
                        color.set(color_art::Color::from_rgb(r.clamp(0.0, 255.0), old.green() as f64, old.blue() as f64).expect("We have a problem!"));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({rgb.0 as f32 / 255.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "val-g val",
                onmounted: move |e| {
                    green_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![green_e, color];
                    async move {
                        let g = e.element_coordinates().x / green_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = color.get().clone();
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
                        let old = color.get().clone();
                        color.set(color_art::Color::from_rgb(old.red() as f64, g.clamp(0.0, 255.0), old.blue() as f64).expect("We have a problem!"));
                    }
                },
                div {
                    class: "knob",
                    style: "left: min({rgb.1 as f32 / 255.0 * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            },
            div {
                class: "val-b val",
                onmounted: move |e| {
                    blue_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![blue_e, color];
                    async move {
                        let b = e.element_coordinates().x / blue_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width * 255.0;
                        let old = color.get().clone();
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
                        let old = color.get().clone();
                        color.set(color_art::Color::from_rgb(old.red() as f64, old.green() as f64, b.clamp(0.0, 255.0)).expect("We have a problem!"));
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
pub fn PanTiltWidget<'a>(cx: Scope<'a>, initial: (f32, f32), onchange: EventHandler<'a, (f32, f32)>) -> Element<'a> {
    let pt = use_state(cx, || initial.clone());

    let pan = use_memo(cx, (pt, ), |(p, )| {
        p.0
    });

    let tilt = use_memo(cx, (pt, ), |(t, )| {
        t.1
    });

    let e = use_effect(cx, (pt, ), |(pt, )| {
        let pt = pt.get().clone();
        onchange.call(pt);
        async move {}
    });

    let mut pan_e = use_state(cx, || None);
    let mut tilt_e = use_state(cx, || None);
    let mut zone_e = use_state(cx, || None);

    cx.render(rsx! {
        div {
           class: "pan-tilt-widget",
            div {
                class: "d2-zone",
                style: "--line-x: min({*tilt * 100.0}%, 100%); --line-y: min({(1.0 - *pan) * 100.0}%, 100%);",
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
            },
            div {
                class: "val-pan",
                onmounted: move |e| {
                    pan_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![pan_e, pt];
                    async move {
                        let p = e.element_coordinates().y / pan_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.height;
                        let old = pt.get().clone();
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
                        let old = pt.get().clone();
                        pt.set((p.clamp(0.0,1.0) as f32, old.1));
                    }
                },
                div {
                    class: "knob",
                    style: "top: min({*pan * 100.0}%, calc(100% - 0.5rem)); pointer-events: none;",
                }
            }

            div {
                class: "val-tilt",
                onmounted: move |e| {
                    tilt_e.set(Some(e.data));
                },
                onclick: move |e| {
                    to_owned![tilt_e, pt];
                    async move {
                        let t = e.element_coordinates().x / tilt_e.get().as_ref().unwrap().get_client_rect().await.unwrap().size.width;
                        let old = pt.get().clone();
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
                        let old = pt.get().clone();
                        pt.set((old.0, t.clamp(0.0,1.0) as f32));
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
