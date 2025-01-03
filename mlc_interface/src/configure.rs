use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdClock, Icon};
use mlc_common::{
    patched::UniverseId, universe::UNIVERSE_SIZE, FaderUpdateRequest, ProjectDefinition,
    RuntimeUpdate,
};

use crate::{
    components::{Fader, Panel, TabOrientation, Tabs},
    utils::{fetch, subscribe_ws},
};

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
        Panel { pos_x: (4, 13), pos_y: (9, 13), ident: "faders", Faders {} }
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
    });
    let keys = use_memo(move || data.read().keys().cloned().collect::<Vec<_>>());
    rsx! {
        Tabs {
            keys: keys(),
            orientation: TabOrientation::Horizontal,
            key_display: move |k: &str| k.to_string(),
            content: move |k| {
                rsx! {
                    {data().get(k).map(|e| *e).unwrap_or_default().to_string()}
                }
            },
        }
    }
}

#[component]
fn Faders() -> Element {
    let universes = use_resource(|| async move {
        fetch::<Vec<u16>>("/data/universes")
            .await
            .unwrap_or_default()
    })
    .suspend()?;
    rsx! {
        Tabs {
            keys: universes(),
            orientation: TabOrientation::Vertical,
            key_display: move |k: u16| k.to_string(),
            content: move |k| {
                rsx! {
                    FaderContainer { universe_id: k }
                }
            },
        }
    }
}

#[component]
fn FaderContainer(universe_id: u16) -> Element {
    let mut fader_values = use_signal(|| [0_u8; UNIVERSE_SIZE]);
    subscribe_ws::<RuntimeUpdate, ()>(
        "/runtime/fader-values/get",
        EventHandler::new(move |u| match u {
            RuntimeUpdate::Universe {
                universe, values, ..
            } => {
                if universe.0 == universe_id {
                    *fader_values.write() = values;
                }
            }
            RuntimeUpdate::ValuesUpdated {
                universes,
                channel_indexes,
                values,
            } => {
                for (i, u_id) in universes.iter().enumerate() {
                    if u_id.0 == universe_id {
                        fader_values.write()[channel_indexes[i]] = values[i];
                    }
                }
            }
            RuntimeUpdate::ValueUpdated {
                universe,
                channel_index,
                value,
            } => {
                if universe.0 == universe_id {
                    fader_values.write()[channel_index] = value;
                }
            }
        }),
    );
    let ws = subscribe_ws::<(), FaderUpdateRequest>(
        "/runtime/fader-values/set",
        EventHandler::new(|_| {}),
    );
    rsx! {
        for i in 0..UNIVERSE_SIZE {
            Fader {
                value: fader_values.map(move |v| &v[i]),
                onchange: move |v| {
                    fader_values.write()[i] = v;
                    ws.send(FaderUpdateRequest {
                        universe: UniverseId(universe_id),
                        channel: i.into(),
                        value: v,
                    });
                },
            }
        }
    }
}
