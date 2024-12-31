use components::ModalMode;
use components::{IconButton, Modal, ToastAdditions};
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdFilePlus, LdLamp};
use dioxus_toast::{ToastFrame, ToastManager};
pub mod components;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Index {},
    #[route("/projects")]
    Projects { },
    #[route("/viewer")]
    Viewer {}
}

const FAVICON: Asset = asset!("/assets/icon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TOAST_CSS: Asset = asset!("/assets/styles/toasts.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let toast = use_context_provider(|| Signal::new(ToastManager::new(10)));
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TOAST_CSS }
        ToastFrame { manager: toast, style: "" }
        Router::<Route> {}
    }
}

#[component]
pub fn Index() -> Element {
    rsx! {}
}

const PROJECTS_STYLES: Asset = asset!("/assets/styles/projects.css");

#[component]
fn Projects() -> Element {
    let mut toast: Signal<ToastManager> = use_context();
    rsx! {
        document::Link { rel: "stylesheet", href: PROJECTS_STYLES }
        div { class: "projectsPage",
            nav {
                MlcBranding {}
                h1 { class: "title", "Projects" }
                div { class: "actions",
                    IconButton {
                        icon: LdFilePlus,
                        onclick: move |_| {
                            toast.success("Happ", "This works");
                        },
                        popovertarget: "create-project",
                    }
                }
            }

            Modal {
                id: "create-project",
                heading: "Test Modal",
                icon: LdLamp,
                mode: ModalMode::Auto,
                onconfirm: move |_| {
                    toast.info("Modal", "Closed with success!");
                },
                oncancel: move |_| {
                    toast.warn("Modal", "Cenceled!");
                },
                h1 { "Hello World" }
                p { "I am a Modal wuhuu!" }
            }
        }
    }
}
#[component]
pub fn Viewer() -> Element {
    rsx! {}
}

#[component]
pub fn MlcBranding() -> Element {
    rsx! {
        div { class: "mlcBranding",
            img { src: FAVICON }
            h1 { "MLC" }
        }
    }
}
