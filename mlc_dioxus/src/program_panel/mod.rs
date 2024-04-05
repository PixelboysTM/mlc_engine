use dioxus::prelude::*;

use crate::icons;

#[component]
pub fn ProgramPanel() -> Element {
    use_context_provider::<Signal<Option<Effect>>>(|| Signal::new(None));

    let mut effect_browser_out = use_signal(|| true);

    rsx! {
        div {
            class: "program-panel",
            class: if !effect_browser_out() {"no-browser"},
            if effect_browser_out() {
                div {
                    class: "panel effect-browser",
                    h3 {
                        class: "header",
                        "Effect Browser",
                    },
                    button {
                        class: "icon close-browser-btn",
                        onclick: move |_| {
                          effect_browser_out.set(false);
                        },
                        icons::PanelLeftClose{},
                    }
                }
            }

            if !effect_browser_out() {
                div {
                    class: "effect-browser-open-btn",
                    onclick: move |_| {
                        effect_browser_out.set(true);
                    },
                    icons::PanelLeftOpen {}
                }
            }

            div {
                class: "panel effect-info",
                "Effect Info",
            },
            div {
                class: "panel timeline",
                "Timeline"
            },
            div {
                class: "panel visualizer",
                "Visualizer"
            }
        }
    }
}