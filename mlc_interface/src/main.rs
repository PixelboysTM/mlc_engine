use components::{DissconnetHelper, IconButton, Modal, ToastAdditions, LATENCY};
use components::{ModalMode, Spinner};
use configure::ConfigurePage;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{
    LdClock, LdFileArchive, LdFileJson, LdFilePlus, LdLightbulb, LdPencil, LdSave, LdSettings,
    LdSquarePlus,
};
use dioxus_free_icons::Icon;
use dioxus_toast::{ToastFrame, ToastManager};
use gloo::storage::Storage;
use mlc_common::{to_save_file_name, CreateProjectData, Info, ProjectDefinition};
use serde::{Deserialize, Serialize};
use utils::{fetch, post, reload_window, subscribe_ws, ToErrToast};
pub mod components;
mod configure;
pub mod utils;

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
    dioxus::logger::init(dioxus::logger::tracing::Level::WARN).expect("Failed to init logging");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut toast = use_context_provider(|| Signal::new(ToastManager::new(10)));
    let mut info = use_context_provider(|| Signal::new(Info::None));
    subscribe_ws::<Info, ()>(
        "/data/info",
        EventHandler::new(move |i| {
            match i {
                Info::ProjectSaved => {
                    toast.success("Saved", "Project saved successfully!");
                }
                Info::ProjectLoaded => {
                    toast.success("Project Loaded", "Loaded successfully. Reloading...");
                    let _ = reload_window();
                }
                Info::SystemShutdown => toast.info("Shutdown", "System is shutting down."),
                Info::RequireReload => {
                    toast.warn("Realod", "Host requested a reload. Acting acordingly!");
                    let _ = reload_window().map_err(|e| toast.error("Failed reload", e));
                }
                Info::UniversePatchChanged(_)
                | Info::EndpointConfigChanged
                | Info::EffectListChanged
                | Info::UniversesUpdated
                | Info::FixtureTypesUpdated
                | Info::None => {}
            }
            log!("Got Info", format!("{i:?}"));
            info.set(i);
        }),
    );

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TOAST_CSS }
        ToastFrame { manager: toast, style: "" }
        Router::<Route> {}
    }
}

const EDITOR_SYTLES: Asset = asset!("/assets/styles/editor.css");

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Tab {
    Configure,
    Program,
    Show,
}

const TAB_STORAGE_KEY: &str = "lastTab";

#[component]
pub fn Index() -> Element {
    let r = use_resource(|| async move {
        fetch::<ProjectDefinition>("/projects/current")
            .await
            .expect("Why no data?")
    });
    let mut data = use_context_provider(|| r);

    let info: Signal<Info> = use_context();

    use_effect(move || {
        if info() == Info::ProjectSaved || info() == Info::ProjectLoaded {
            data.restart();
        }
    });

    let mut tab = use_signal(|| {
        gloo::storage::LocalStorage::get::<Tab>(TAB_STORAGE_KEY).unwrap_or(Tab::Configure)
    });

    use_effect(move || {
        let new_tab = *tab.read();
        let _ = gloo::storage::LocalStorage::set(TAB_STORAGE_KEY, new_tab);
    });
    rsx! {
        document::Stylesheet { href: EDITOR_SYTLES }
        div { class: "editorPage",
            nav {
                MlcBranding {}
                div { class: "tabs",
                    IconButton {
                        icon: LdSettings,
                        style: if tab() == Tab::Configure { "color: var(--c-p" },
                        onclick: move |_| tab.set(Tab::Configure),
                    }
                    IconButton {
                        icon: LdPencil,
                        style: if tab() == Tab::Program { "color: var(--c-s" },
                        onclick: move |_| tab.set(Tab::Program),
                    }
                    IconButton {
                        icon: LdLightbulb,
                        style: if tab() == Tab::Show { "color: var(--c-t" },
                        onclick: move |_| tab.set(Tab::Show),
                    }
                }
                div { class: "actions",
                    match tab() {
                        Tab::Configure => rsx! {},
                        Tab::Program => rsx! {},
                        Tab::Show => rsx! {},
                    }
                    IconButton {
                        icon: LdSave,
                        onclick: move |_| async move {
                            let _ = fetch::<()>("/data/save").await;
                        },
                    }
                }
            }

            div { class: "content",
                match tab() {
                    Tab::Configure => rsx! {
                        ConfigurePage {}
                    },
                    Tab::Program => rsx! { "Program" },
                    Tab::Show => rsx! { "Show" },
                }
            }

            footer {
                code { style: "text-align: left;", "Latency {LATENCY()}ms" }
                match data() {
                    None => rsx! {
                        code { style: "text-align: center;", "..." }
                    },
                    Some(d) => rsx! {
                        code { style: "text-align: center;", {d.name.clone()} }
                    },
                }
                DissconnetHelper {}
            }
        }
    }
}

const PROJECTS_STYLES: Asset = asset!("/assets/styles/projects.css");

#[component]
fn Projects() -> Element {
    let mut toast: Signal<ToastManager> = use_context();

    let mut new_project_name = use_signal(|| "New Project".to_string());
    let mut file_name = use_signal(|| to_save_file_name("New Project"));
    let mut binary = use_signal(|| true);
    use_effect(move || {
        file_name.set(to_save_file_name(&*new_project_name.read()));
    });
    rsx! {
        document::Stylesheet { href: PROJECTS_STYLES }
        div { class: "projectsPage",
            nav {
                MlcBranding {}
                h1 { class: "title", "Projects" }
                div { class: "actions",
                    IconButton { icon: LdFilePlus, popovertarget: "create-project" }
                }
            }

            //plist
            SuspenseBoundary {
                fallback: |_context: SuspenseContext| rsx! {
                    Spinner {}
                },
                ProjectList {}
            }

            Modal {
                id: "create-project",
                heading: "Create Project",
                icon: LdSquarePlus,
                mode: ModalMode::Manual,
                confirm_text: "Create",
                onconfirm: move |_| async move {
                    let _ = post::<
                        String,
                        _,
                    >(
                            "/projects/create",
                            CreateProjectData {
                                name: new_project_name.read().clone(),
                                binary: *binary.read(),
                            },
                        )
                        .await;
                },
                label { class: "input",
                    span { "    Name: " }
                    input {
                        value: new_project_name,
                        oninput: move |e| {
                            new_project_name.set(e.value());
                        },
                    }
                }
                label { class: "input",
                    span { "Filename: " }
                    code { {file_name()} }
                }
                label { class: "input",
                    span { "  Format: " }
                    select {
                        name: "Format",
                        onchange: move |e| {
                            let val = e.value();
                            if &val == "json" {
                                binary.set(false);
                            } else if &val == "binary" {
                                binary.set(true);
                            } else {
                                toast
                                    .warn(
                                        "Missing option",
                                        "The selected option is not valid. Ho did that happened?",
                                    );
                            }
                        },
                        option { value: "json", "Json" }
                        option { value: "binary", "Binary" }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectList() -> Element {
    let mut toast: Signal<ToastManager> = use_context();
    let projects = use_resource(move || async move {
        let mut pjs = fetch::<Vec<ProjectDefinition>>("/projects/projects-list")
            .await
            .catch_toast(
                &mut toast,
                "Project List",
                "Failed to retrieve project list:\n",
                Vec::new(),
            );
        pjs.sort_by(|a, b| a.last_edited.cmp(&b.last_edited).reverse());
        pjs
    })
    .suspend()?;
    rsx! {
        div { class: "projectList",
            for project in projects().clone() {
                div {
                    class: "project",
                    class: if project.binary { "binary" },
                    tabindex: 0,
                    ondoubleclick: move |_| {
                        let n = project.file_name.clone();
                        async move {
                            let u = fetch::<String>(&format!("/projects/load/{}", n)).await;
                            if let Err(_) = u {
                                toast
                                    .error(
                                        "Project Loading",
                                        "An error occured when trying to load the project.",
                                    );
                            }
                        }
                    },
                    h1 { {project.name.clone()} }
                    p {
                        Icon { icon: LdClock }
                        {project.last_edited.format("%d.%m.%Y %H:%M").to_string()}
                    }
                    code { {project.file_name.clone()} }
                    if project.binary {
                        Icon { class: "icon", icon: LdFileArchive }
                    } else {
                        Icon { class: "icon", icon: LdFileJson }
                    }
                }
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
