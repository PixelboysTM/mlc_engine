use std::time::Duration;

use dioxus::prelude::*;
use futures::StreamExt;
use gloo_net::websocket::Message;
use gloo_storage::Storage;
use mlc_common::Info;
use wasm_logger::Config;

use crate::configure_panel::ConfigurePanel;
use crate::headbar::{Headbar, Pane};
use crate::utils::context_menu::ContextMenu;
use crate::utils::Loading;
use crate::utils::toaster::{Toaster, ToasterWriter};

pub(crate) mod configure_panel;
mod headbar;
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

    let pane = use_signal(|| gloo_storage::LocalStorage::get::<Pane>("lastTab").unwrap_or(Pane::Program));


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
    let mut toaster = use_context::<Signal<Toaster>>();

    let mut c_menu = use_signal(|| None);
    rsx! {
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
                            toaster.info("Test Info!", "This is a test notification and will be discarded shortly!");
                        },
                        "Toast Info"
                        },
                        button {
                            onclick: move |_| {
                                toaster.warning("Test Warning!", "This is a warning be carefull!");
                            },
                            "Toast Warning"
                        },
                        button {
                            onclick: move |_| {
                                toaster.error("An error occured", "This is a error message someting went wrong and it is your job to figure out what here is some context: Lorem ipsum dhuaijkdbasjdbahssdjd you are fucked!!!!!");
                            },
                            "Toast Error"
                        }
                        button {
                            onclick: move |_| {
                                toaster.log("Log", "This is just a bit of logging");
                            },
                            "Toast Log"
                        }
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
                        toaster.log("Universe Patch changed", format!("Universe {} changed", u.0));
                    }
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
    });
}
