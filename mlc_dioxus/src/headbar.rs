use std::ops::Deref;

use dioxus::core::{Element, Scope};
use dioxus::prelude::*;
use log::log;

use crate::icons::{
    ExternalLink, LightBulb, Pencil, Save, Settings, TabletSmartphone, UploadCloud,
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Pane {
    Configure,
    Program,
    Show,
}

#[component]
pub fn Headbar(cx: Scope) -> Element {
    let pane = use_shared_state::<Pane>(cx).unwrap();

    render! {
        div {
            class: "headbar",
            img {
                class: "iconMarvin",
                src: "./images/icon.png",
                alt: "MLC",
            }
            div {
                class: "tabs",
                button {
                    class: "icon configure {sel(pane.read().deref() == &Pane::Configure)}",
                    title: "Configure",
                    onclick: move |_event| {
                        *pane.write() = Pane::Configure;
                        log::info!("Clicked Configure");
                    },
                    Settings {}
                }
                button {
                    class: "icon program {sel(pane.read().deref() == &Pane::Program)}",
                    title: "Program",
                    onclick: move |_event| {
                        *pane.write() = Pane::Program;
                        log::info!("Clicked Program")
                    },
                    Pencil {}
                }
                button {
                    class: "icon show {sel(pane.read().deref() == &Pane::Show)}",
                    title: "Show",
                    onclick: move |_event| {
                        *pane.write() = Pane::Show;
                        log::info!("Clicked Show")
                    },
                    LightBulb {}
                }
            }
            div {
                class: "tabs right",

                if pane.read().deref() == &Pane::Configure {
                    rsx! {
                    button {
                        class: "icon",
                        title: "Upload Fixture",
                        onclick: move |_event| {
                            log::info!("Clicked Save")
                        },
                        UploadCloud {},
                    },
                    }
                }

                if pane.read().deref() == &Pane::Program {
                    rsx! {
                    button {
                        class: "icon",
                        title: "Open 3D Viewer",
                        onclick: move |_event| {
                            log::info!("Clicked Save")
                        },
                        ExternalLink {},
                    },
                    }
                }

                if pane.read().deref() == &Pane::Show {
                    rsx! {
                    button {
                        class: "icon",
                        title: "Open 3D Viewer",
                        onclick: move |_event| {
                            log::info!("Clicked Save")
                        },
                        TabletSmartphone {},
                    },
                    }
                }

                button {
                    class: "icon",
                    title: "Save Project",
                    onclick: move |_event| {
                        log::info!("Clicked Save")
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

fn sel(b: bool) -> &'static str {
    if b {
        "sel"
    } else {
        ""
    }
}
