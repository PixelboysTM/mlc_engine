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
    cx.render(rsx! {
        div {
            class: "loading-spinner",
            div {
                class: "inner",
            }
        }
    })
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
pub fn RgbWidget(cx: Scope, initial: (f32, f32, f32)) -> Element {
    // let initial: (f32, f32, f32) = (1.0, 1.0, 1.0);
    // let i = color_art::Color::from_rgb(initial.0 * 255.0, initial.1 * 255.0, initial.2 * 255.0).unwrap();
    let c = use_state(cx, || color_art::Color::from_rgb(initial.0 * 255.0, initial.1 * 255.0, initial.2 * 255.0).unwrap());

    let hsv = use_memo(cx, (c, ), |(c, )| {
        (c.hue(), c.hsv_value(), c.hsv_value())
    });

    let hue_col = use_memo(cx, (c, ), |(c, )| {
        color_art::Color::from_hsv(c.hue(), 1.0, 1.0).unwrap().hex()
    });

    cx.render(rsx! {
        div {
                class: "rgb-widget",
                div {
                    class: "sat",
                    style: "background: linear-gradient(90deg, {hue_col.to_string()}, white), linear-gradient(0deg, black, white);",
                },
                div {
                    class: "hue"
                },

                div {
                    class: "val-r"
                },
                div {
                    class: "val-g"
                },
                div {
                    class: "val-b"
                }
            }
    })
}
