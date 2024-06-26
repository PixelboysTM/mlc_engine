use std::time::Duration;

use dioxus::prelude::*;
use futures::StreamExt;
use gloo_net::websocket::Message;
use gloo_storage::Storage;
use wasm_logger::Config;

use mlc_common::Info;

use crate::configure_panel::ConfigurePanel;
use crate::head_bar::{Headbar, Pane};
use crate::program_panel::ProgramPanel;
use crate::utils::context_menu::ContextMenu;
use crate::utils::Loading;
use crate::utils::popover::Popover;
use crate::utils::toaster::{Toaster, ToasterWriter};

pub(crate) mod configure_panel;
pub(crate) mod program_panel;
mod head_bar;
pub mod icons;
mod project_selection;
mod utils;

fn main() {
    wasm_logger::init(Config::default());
    launch(root);
}

fn root() -> Element {
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
    #[route("/viewer")]
    Viewer {}
}

#[allow(non_snake_case)]
fn Projects() -> Element {
    provide_info();
    rsx! {
        project_selection::ProjectSelection{}
    }
}

#[allow(non_snake_case)]
fn Index() -> Element {
    provide_info();

    let pane =
        use_signal(|| gloo_storage::LocalStorage::get::<Pane>("lastTab").unwrap_or(Pane::Program));

    use_effect(move || {
        gloo_storage::LocalStorage::set("lastTab", pane()).expect("Writing failed");
    });

    rsx! {
        DisconnectHelper {},
        Headbar{
            pane,
        },
        IndexContent {
            pane,
        }

    }
}

#[component]
fn IndexContent(pane: Signal<Pane>) -> Element {
    let mut c_menu = use_signal(|| None);
    let mut popover = use_signal(|| false);
    rsx! {
        div {
            width: "100vw",
            height: "calc(100vh - 3rem)",
            match pane() {
                Pane::Configure => {
                    rsx! {
                        ConfigurePanel{}
                    }
                }
                Pane::Program => {
                    rsx! {
                        ProgramPanel{}
                    }
                }
                Pane::Show => {
                    rsx!{
                        "Show",
                        button {
                            onclick: move |e| {
                                let p = e.data().page_coordinates();
                                c_menu.set(Some(ContextMenu::new(p.x, p.y).add("Item 1 mit Action", |_| {log::info!("Item 1 Clicked"); true}).add("Item 2", |_| {log::info!("Item 2 Clicked"); true})));
                            },
                            "Context",
                            if let Some(m) = c_menu() {
                                ContextMenu {
                                    menu: m,
                                    onclose: move |_| {
                                        log::info!("Context Menu closing");
                                        c_menu.set(None);
                                    }
                                }
                            }
                        },
                        button {
                            onclick: move |_| {
                                popover.set(true);
                            },
                            "Popover",
                            if popover() {
                                Popover {
                                    class: "test-popover",
                                    onclose: move |_| {
                                        popover.set(false);
                                    },
                                    "Popover",
                                    button {
                                        onclick: move |_| {
                                            log::info!("CLike die click")
                                        },
                                        "Blip"
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
fn DisconnectHelper() -> Element {
    let info = use_context::<Signal<Info>>();
    let mut active = use_signal(|| false);
    use_effect(move || {
        if info() == Info::SystemShutdown {
            active.set(true);
        }
    });

    let _guard = use_future(move || async move {
        let mut failed = 0;
        while failed <= 3 && !active() {
            let r = utils::fetch::<String>("/util/heartbeat").await;
            if r.is_ok() {
                async_std::task::sleep(Duration::from_secs(5)).await;
                failed = 0;
            } else {
                failed += 1;
                log::warn!("Failed heartbeat {} times", failed);
            }
        }

        active.set(true);
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
                            utils::reload_window().expect("");
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

fn provide_info() {
    let mut info = provide_root_context(Signal::new(Info::None));
    use_future(move || async move {
        let mut toaster = use_context::<Signal<Toaster>>();

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
                        utils::toast_reload(toaster);
                    }
                    Info::SystemShutdown => {
                        toaster.info("Shutting down", "MLC is exiting");
                    }
                    Info::RequireReload => {
                        utils::toast_reload(toaster);
                    }
                    Info::FixtureTypesUpdated => {}
                    Info::UniversePatchChanged(u) => {
                        toaster.log(
                            "Universe Patch changed",
                            format!("Universe {} changed", u.0),
                        );
                    }
                    Info::UniversesUpdated => {}
                    Info::EndpointConfigChanged => {
                        toaster.info(
                            "Endpoint Config chnaged",
                            "The Endpoint configuration was changed!",
                        );
                    }
                    Info::EffectListChanged => {}
                    Info::None => {}
                }
            }
            log::error!("Error with msg");
        } else {
            log::info!("Error creating ws {:?}", ws.err().unwrap());
        }
    });
}

#[component]
fn Viewer() -> Element {
    rsx! {
        iframe {
            src: "/iviewer",
            style: "width: 100vw; height: 100vh",
        }
    }
}
