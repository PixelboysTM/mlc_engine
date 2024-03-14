use crate::{icons, utils};
use dioxus::core::Scope;
use dioxus::prelude::*;
use mlc_common::ProjectDefinition;

#[component]
pub fn ProjectSelection(cx: Scope) -> Element {
    let projects = use_future(cx, (), |_| async move {
        utils::fetch::<Vec<ProjectDefinition>>("/projects/projects-list")
            .await
            .map_err(|e| log::error!("Error Loading Project list: {e}"))
            .ok()
    });

    cx.render(rsx! {
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
                    "Projects"
                }
            }

            div {
                class: "tabs right",

                button {
                    class: "icon",
                    title: "Import Project",
                    onclick: move |_event| {
                        log::info!("Import Project")
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
            match projects.value() {
                Some(Some(ps)) => {
                    cx.render(rsx!{
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
                                        let eval = use_eval(cx);

                                        to_owned![eval];
                                        async move {
                                            let u = utils::fetch::<String>(&format!("/projects/load/{}", n)).await;
                                            if u.is_ok() {
                                                let _ = eval("window.location.reload()");
                                            } else {
                                                log::error!("Error opening project: {:?}", u.err().unwrap());
                                            }
                                        }
                                    },
                                    icons::FolderOpen {}

                                }

                            }
                        }
                    })
                }
                Some(None) => {cx.render(rsx!("Error loading project list!"))}
                None => {cx.render(rsx!{utils::Loading {}})}
            }
        }
    })
}
