use dioxus::prelude::*;

use mlc_common::{CreateProjectData, ProjectDefinition};

use crate::{icons, utils};
use crate::utils::toaster::{Toaster, ToasterWriter};

#[component]
pub fn ProjectSelection() -> Element {
    let projects: Resource<Option<Vec<ProjectDefinition>>> = use_resource(|| async move {
        utils::fetch::<Vec<ProjectDefinition>>("/projects/projects-list")
            .await
            .map_err(|e| log::error!("Error Loading Project list: {e}"))
            .map(|mut s| {
                s.sort_by(|a, b| a.last_edited.cmp(&b.last_edited));
                s.reverse();
                s
            })
            .ok()
    });

    let mut toaster = use_context::<Signal<Toaster>>();

    let mut create_project = use_signal(|| false);
    let mut new_project_name = use_signal(|| "New Project".to_string());
    let mut new_project_binary = use_signal(|| false);

    rsx! {
        if create_project(){
            utils::Overlay {
                title: "Create New Project",
                class: "new-project",
                icon: rsx!(icons::PencilRuler{}),
                onclose: move |_| {
                    create_project.set(false);
                    new_project_name.set("New Project".to_string());
                    new_project_binary.set(false);
                },
                p {
                    class: "name-title",
                    "Project Name:"
                }
                input {
                    class: "name",
                    r#type: "text",
                    onchange: move |e| {
                        new_project_name.set(e.value());
                    },
                    oninput: move |e| {
                        new_project_name.set(e.value());
                    },
                    value: new_project_name(),
                },
                p {
                    class: "file",
                    "Will be saved as: ",
                    span {
                        {mlc_common::to_save_file_name(&new_project_name())}
                    }
                },
                p {
                    class: "binary-toggle",
                    "Binary format: ",
                    utils::Toggle {
                        value: new_project_binary,
                    }
                }
                div {
                    class: "buttons",
                    button {
                        class: "create-btn",
                        onclick: move |_| {
                            async move {
                                if !new_project_name.peek().is_empty() {
                                    let result = utils::fetch_post::<String, _>("/projects/create", CreateProjectData{
                                        name: new_project_name.peek().clone(),
                                        binary: *new_project_binary.peek(),
                                    }).await;
                                    if let Ok(_) = result {
                                        utils::toast_reload(toaster);
                                    } else {
                                        log::error!("{result:?}");
                                        toaster.error("Project creation failed!", "Failed to create new project. See the console for more detailed information.");
                                    }
                                }
                            }
                        },
                        "Create"
                    },
                    button {
                        class: "cancel-btn",
                        onclick: move |_| {
                            create_project.set(false);
                            new_project_name.set("New Project".to_string());
                    new_project_binary.set(false);
                        },
                        "Cancel"
                    }
                }
            }
        }

        div {
            class: "headbar project-bar",
            div {
                class: "left",
                onclick: move |_| {
                   toaster.info("Hello", "**zwitscher**");
                },
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
                        log::info!("New Project");
                        create_project.set(true);
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
                                div {
                                    class: "bin-ico",
                                    title: if p.binary {"Compressed"} else {"Uncompressed"},
                                {if p.binary {
                                    rsx! {
                                        icons::FileArchive {}
                                    }
                                } else {
                                    rsx!{
                                        icons::FileJson {}
                                    }
                                }}
                                }

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
                                                utils::toast_reload(toaster);
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
