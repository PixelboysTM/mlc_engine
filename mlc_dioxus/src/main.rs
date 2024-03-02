use std::ops::Deref;
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_router::prelude::{Routable, Router};
use futures::StreamExt;
use gloo_net::websocket::Message;
use reqwest::{IntoUrl, Url};
use wasm_logger::Config;
use mlc_common::Info;

use crate::configure_panel::ConfigurePanel;
use crate::headbar::{Headbar, Pane};
use crate::utils::Loading;

mod headbar;
mod project_selection;
pub mod icons;
pub(crate) mod configure_panel;
mod utils;

fn main() {
    wasm_logger::init(Config::default());
    dioxus_web::launch(start);
}

fn start(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}


#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Index {},
    #[route("/projects")]
    Projects {},
}

#[allow(non_snake_case)]
fn Index(cx: Scope) -> Element {
    app(cx)
}

#[allow(non_snake_case)]
fn Projects(cx: Scope) -> Element {
    project_selection::ProjectSelection(cx)
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Pane::Configure);
    let pane = use_shared_state::<Pane>(cx).unwrap();

    let info = use_state(cx, || Info::None);

    let started = use_state(cx, || false);
    let create_eval = use_eval(cx);
    let info_watcher = use_future(
        cx,
        (),
        |_| {
            let eval = create_eval(r#"dioxus.send(window.location.host)"#).unwrap();

            to_owned![info, started];
            async move {
                if *started.get() {
                    return;
                }
                started.set(true);
                log::info!("Started");

                let ws_url = &format!(
                    "ws://{}/data/info",
                    eval.recv()
                        .await
                        .map_err(|e| log::error!("Error"))
                        .unwrap()
                        .as_str()
                        .unwrap()
                );

                let mut ws = utils::ws(ws_url);
                if let Ok(mut ws) = ws {
                    while let Some(Ok(msg)) = ws.next().await {
                        let msg = match msg {
                            Message::Text(t) => { t }
                            Message::Bytes(b) => { String::from_utf8(b).unwrap() }
                        };

                        let i = serde_json::from_str::<Info>(&msg).unwrap();
                        info.set(i);

                        log::info!("Updating");
                    }
                    log::error!("Error with msg");
                } else {
                    log::info!("Error creating ws {:?}", ws.err().unwrap());
                }
            }
        },
    );


    cx.render(rsx! {
        DisconnectHelper {
            info: {info.get().clone()}
        },
        Headbar{},
        div {
            width: "100vw",
            height: "calc(100vh - 3rem)",
            match pane.read().deref() {
                Pane::Configure => {
                    ConfigurePanel(cx)
                }
                Pane::Program => {
                    cx.render(rsx!{
                        "Program"
                    })
                }
                Pane::Show => {
                    cx.render(rsx!{
                        "Show"
                    })
                }
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct DHProps {
    info: Info,
}

#[component]
fn DisconnectHelper(cx: Scope<DHProps>) -> Element {
    let active = use_state(cx, || false);
    use_memo(cx, &(cx.props.info, ), |(i, )| {
        if i == Info::SystemShutdown {
            active.set(true);
        }
    });

    // active.set(*info.read() == Info::SystemShutdown);
    let eval = use_eval(cx);

    let guard = use_future(cx, (), |_| {
        to_owned![active];
        async move {
            let mut failed = 0;
            while failed <= 5 {
                let r = utils::fetch::<String>("/util/heartbeat").await;
                if r.is_ok() {
                    async_std::task::sleep(Duration::from_secs(5)).await;
                } else {
                    failed += 1;
                    log::warn!("Failed heartbeat {} times", failed);
                }
            }

            active.set(true);
        }
    });

    if *active.get() {
        cx.render(rsx! {
            div {
                class: "disconnect-helper overlay",
                div {
                    class: "overlay-content",
                    h3 {
                        "Backend shutdown please restart and reload!"
                    },
                    Loading {},
                    button {
                        onclick: move |e| {
                            let _ = eval("window.location.reload()");
                        },
                        "Reload"
                    }
                }
            }
        })
    } else {
        cx.render(rsx! {
            ""
        })
    }
}
