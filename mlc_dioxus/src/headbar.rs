use dioxus::prelude::*;

use crate::icons::{
    ExternalLink, LightBulb, Pencil, Save, Settings, TabletSmartphone, UploadCloud,
};
use crate::{configure_panel, utils};

#[derive(Copy, Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Pane {
    Configure,
    Program,
    Show,
}

#[component]
pub fn Headbar(pane: Signal<Pane>) -> Element {
    let mut upload_fixture = use_signal(|| false);

    rsx! {
        if upload_fixture() {
            configure_panel::UploadFixturePopup {
                on_close: move |_| {
                    upload_fixture.set(false);
                }
            }
        }

        div {
            class: "headbar",
            div {
                class: "left",
                img {
                    class: "iconMarvin",
                    src: "./images/icon.png",
                    alt: "MLC",
                },
                h1 {
                    "MLC"
                }
            }
            div {
                class: "tabs",
                button {
                    class: "icon configure",
                    class: if pane() == Pane::Configure {"sel"},
                    title: "Configure",
                    onclick: move |_event| {
                        pane.set(Pane::Configure);
                        log::info!("Clicked Configure");
                    },
                    Settings {}
                }
                button {
                    class: "icon program",
                    class: if pane() == Pane::Program {"sel"},
                    title: "Program",
                    onclick: move |_event| {
                        pane.set(Pane::Program);
                        log::info!("Clicked Program")
                    },
                    Pencil {}
                }
                button {
                    class: "icon show",
                    class: if pane() == Pane::Show {"sel"},
                    title: "Show",
                    onclick: move |_event| {
                        pane.set(Pane::Show);
                        log::info!("Clicked Show")
                    },
                    LightBulb {}
                }
            }
            div {
                class: "tabs right",

                if pane() == Pane::Configure {
                    button {
                        class: "icon",
                        title: "Upload Fixture",
                        onclick: move |_event| {
                                upload_fixture.set(true);
                        },
                        UploadCloud {},
                    }
                }

                if pane() == Pane::Program {
                    button {
                        class: "icon",
                        title: "Open 3D Viewer",
                        onclick: move |_event| {
                            log::info!("Clicked Save")
                        },
                        ExternalLink {},
                    }
                }

                if pane() == Pane::Show {
                    button {
                        class: "icon",
                        title: "Open 3D Viewer",
                        onclick: move |_event| {
                            log::info!("Clicked Save")
                        },
                        TabletSmartphone {},
                    }
                },

                button {
                    class: "icon",
                    title: "Save Project",
                    onclick: move |_event| {
                        async move {
                            let _ = utils::fetch::<String>("/data/save").await;
                        }
                    },
                    Save {}
                },
                div {
                  width: "0.25rem",
                },
            }
        }
    }
}
