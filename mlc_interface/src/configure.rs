use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdClock, Icon};
use mlc_common::ProjectDefinition;

use crate::components::{Panel, TabOrientation, Tabs};

const CONFIGURE_CSS: Asset = asset!("/assets/styles/configure.css");
#[component]
pub fn ConfigurePage() -> Element {
    rsx! {
        document::Stylesheet { href: CONFIGURE_CSS }
        Panel {
            pos_x: (1, 4),
            pos_y: (1, 4),
            ident: "info",
            title: "Info",
            InfoPanel {}
        }
        Panel {
            pos_x: (1, 4),
            pos_y: (4, 13),
            ident: "types",
            title: "Fixture Types",
            "Hello 2"
        }
        Panel {
            pos_x: (4, 10),
            pos_y: (1, 9),
            ident: "patching",
            title: "Universe Explorer",
            UniverseExplorer {}
        }
        Panel { pos_x: (4, 13), pos_y: (9, 13), ident: "faders" }
        Panel {
            pos_x: (10, 13),
            pos_y: (1, 9),
            ident: "settings",
            title: "Project Settings",
        }
    }
}

#[component]
fn InfoPanel() -> Element {
    let data = use_context::<Resource<ProjectDefinition>>().suspend()?;
    rsx! {
        p { {data.read().name.clone()} }
        code {
            {data.read().file_name.clone()}
            if data.read().binary {
                span { "Binary" }
            } else {
                span { "Json" }
            }
        }
        p {
            Icon { icon: LdClock }
            {data.read().last_edited.format("%d.%m.%Y %H:%M").to_string()}
        }
    }
}

#[component]
fn UniverseExplorer() -> Element {
    let mut data = use_signal(|| HashMap::new());
    use_future(move || async move {
        data.write().insert("A (Yes)", "Ananans");
        data.write().insert("B (No)", "Banane");
        data.write().insert("C (Really)", "Chikoreh");
        data.write().insert("D (Why) 1", "Donauwelle");
        data.write().insert("D (Why) 2", "Donauwelle");
        data.write().insert("D (Why) 3", "Donauwelle");
        data.write().insert("D (Why) 4", "Donauwelle");
        data.write().insert("D (Why) 5", "Donauwelle");
        data.write().insert("D (Why) 6", "Donauwelle");
        data.write().insert("D (Why) 7", "Donauwelle");
        data.write().insert("D (Why) 8", "Donauwelle");
        data.write().insert("D (Why) 9", "Donauwelle");
        data.write().insert("D (Why) A", "Donauwelle");
        data.write().insert("D (Why) B", "Donauwelle");
    });
    let keys = use_memo(move || data.read().keys().cloned().collect::<Vec<_>>());
    rsx! {
        Tabs {
            keys: keys(),
            orientation: TabOrientation::VerticalText,
            key_display: move |k: &str| k.to_string(),
            content: move |k| {
                rsx! {
                    {data().get(k).map(|e| *e).unwrap_or_default().to_string()}
                }
            },
        }
    }
}
