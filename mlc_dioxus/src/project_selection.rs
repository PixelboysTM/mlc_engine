use crate::{icons, utils};
use dioxus::prelude::*;
use mlc_common::ProjectDefinition;
use crate::utils::toaster::{Toaster, ToasterWriter};

#[component]
pub fn ProjectSelection() -> Element {
    let projects: Resource<Option<Vec<ProjectDefinition>>> = use_resource(|| async move {
        utils::fetch::<Vec<ProjectDefinition>>("/projects/projects-list")
            .await
            .map_err(|e| log::error!("Error Loading Project list: {e}"))
            .ok()
    });

    let mut toaster = use_context::<Signal<Toaster>>();

    rsx! {
        div {
            class: "headbar project-bar",
            img {
                class: "iconMarvin",
                src: "./images/icon.png",
                alt: "MLC",
            },

            div {
                style: "display: flex; align-items: center;",

                h2 {
                    style: "margin: 0; padding: 0;margin-left: auto; margin-right: auto",
                    "Project Selection"
                }
            }

            div {
                class: "tabs right",

                button {
                    class: "icon",
                    title: "Import Project",
                    onclick: move |_event| {
                        log::info!("Import Project");
                        toaster.info("Unimplemented", "The Upload Project functionality is not yet implemented sorry!");
                    },
                    icons::FileUp {},
                },

                button {
                    class: "icon",
                    title: "Create new Project",
                    onclick: move |_event| {
                        log::info!("New Project")
                    },
                    icons::Plus {},
                },

                div {
                    width: "0.25rem"
                }
            }
        },

        div {
            class: "project-list",
            match projects.value().read().clone() {
                Some(Some(ps)) => {
                    rsx!{
                        for p in ps {
                            div {
                                class: "project",

                                h2 {
                                    class: "name",
                                    {p.name.clone()}
                                },

                                p {
                                    class: "file-name",
                                    {p.file_name.clone()}
                                },

                                p {
                                    class: "last-edited",
                                    {p.last_edited.format("%d.%m.%Y %H:%M:%S").to_string()}
                                }

                                button {
                                    class: "icon",
                                    title: "Open",
                                    onclick: move |_| {
                                        log::info!("Open project");
                                        let n = p.file_name.clone();

                                        async move {
                                            let u = utils::fetch::<String>(&format!("/projects/load/{}", n)).await;
                                            if u.is_ok() {
                                                let _ = eval("window.location.reload()");
                                            } else {
                                                log::error!("Error opening project: {:?}", u.err().unwrap());
                                                toaster.error("Project Loading Error", "An error occurred while loading the project! For more detailed information see the backend log or browser console.");
                                            }
                                        }
                                    },
                                    icons::FolderOpen {}

                                }

                            }
                        }
                    }
                }
                Some(None) => {rsx!("Error loading project list!")}
                None => {rsx!{utils::Loading {}}}
            }
        }
    }
}
