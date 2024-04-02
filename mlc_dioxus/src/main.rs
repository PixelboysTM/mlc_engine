use std::time::Duration;

use dioxus::prelude::*;
use futures::StreamExt;
use gloo_net::websocket::Message;
use gloo_storage::Storage;
use mlc_common::Info;
use wasm_logger::Config;

use crate::configure_panel::ConfigurePanel;
use crate::headbar::{Headbar, Pane};
use crate::utils::Loading;
use crate::utils::toaster::{Toaster, ToasterWriter};

pub(crate) mod configure_panel;
mod headbar;
pub mod icons;
mod project_selection;
mod utils;

fn main() {
    wasm_logger::init(Config::default());
    launch(start);
}

fn start() -> Element {
    utils::toaster::init_toaster();
    rsx! {
        utils::toaster::ToasterElement {},
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
fn Projects() -> Element {
    project_selection::ProjectSelection()
}

#[allow(non_snake_case)]
fn Index() -> Element {
    let pane = use_signal(|| gloo_storage::LocalStorage::get::<Pane>("lastTab").unwrap_or(Pane::Program));

    let mut toaster = use_context::<Signal<Toaster>>();

    use_effect(move || {
        gloo_storage::LocalStorage::set("lastTab", pane()).expect("Writing failed");
    });

    let mut info = use_signal(|| Info::None);

    let mut started = use_signal(|| false);
    // let create_eval =  use_eval(cx);
    let _info_watcher = use_future(move || {
        async move {
            if started() {
                return;
            }
            started.set(true);
            log::info!("Started");

            let ws = utils::ws("/data/info").await;
            if let Ok(mut ws) = ws {
                while let Some(Ok(msg)) = ws.next().await {
                    let msg = match msg {
                        Message::Text(t) => t,
                        Message::Bytes(b) => String::from_utf8(b).unwrap(),
                    };

                    let i = serde_json::from_str::<Info>(&msg).unwrap();
                    info.set(i);

                    match i {
                        Info::ProjectSaved => {
                            toaster.info("Project Saved", "Project saved to disk successfully!");
                        }
                        Info::ProjectLoaded => {
                            toaster.info("Project Loaded", "Project Loaded successfully!");
                        }
                        Info::SystemShutdown => {
                            toaster.info("Shutting down", "MLC is exiting");
                        }
                        Info::FixtureTypesUpdated => {}
                        Info::UniversePatchChanged(_) => {}
                        Info::UniversesUpdated => {}
                        Info::EndpointConfigChanged => {}
                        Info::EffectListChanged => {}
                        Info::None => {}
                    }
                }
                log::error!("Error with msg");
            } else {
                log::info!("Error creating ws {:?}", ws.err().unwrap());
            }
        }
    });

    rsx! {
        DisconnectHelper {
            info
        },
        Headbar{
            pane,
        },
        div {
            width: "100vw",
            height: "calc(100vh - 3rem)",
            match pane() {
                Pane::Configure => {
                    ConfigurePanel()
                }
                Pane::Program => {
                    rsx!{
                        "Program",
                        button {
                            onclick: move |_| {
                                 toaster.write().info("Test Info!", "This is a test notification and will be discarded shortly!"); // TODO: Add Trait to shorten this
                            },
                            "Toast Info"
                        },
                        button {
                            onclick: move |_| {
                                 toaster.write().warning("Test Warning!", "This is a warning be carefull!"); // TODO: Add Trait to shorten this
                            },
                            "Toast Warning"
                        },
                        button {
                            onclick: move |_| {
                                 toaster.write().error("An error occured", "This is a error message someting went wrong and it is your job to figure out what here is some context: Lorem ipsum dhuaijkdbasjdbahssdjd you are fucked!!!!!"); // TODO: Add Trait to shorten this
                            },
                            "Toast Error"
                        }
                        button {
                            onclick: move |_| {
                                 toaster.write().log("Log", "This is just a bit of logging"); // TODO: Add Trait to shorten this
                            },
                            "Toast Log"
                        }
                    }
                }
                Pane::Show => {
                    rsx!{
                        "Show"
                    }
                }
            }
        }
    }
}


#[component]
fn DisconnectHelper(info: Signal<Info>) -> Element {
    let mut active = use_signal(|| false);
    let _ = use_memo(move || {
        if info() == Info::SystemShutdown {
            active.set(true);
        }
    });

    // active.set(*info.read() == Info::SystemShutdown);

    let _guard = use_future(move || {
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

    if active() {
        rsx! {
            div {
                class: "disconnect-helper overlay",
                div {
                    class: "overlay-content",
                    h3 {
                        "Backend shutdown please restart and reload!"
                    },
                    Loading {},
                    button {
                        onclick: move |_| {
                            let _ = eval("window.location.reload()");
                        },
                        "Reload"
                    }
                }
            }
        }
    } else {
        rsx! {
            ""
        }
    }
}