use dioxus::prelude::*;

use crate::icons;

#[component]
pub fn Overlay(
    title: String,
    class: String,
    icon: Element,
    onclose: EventHandler,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "overlay",
            onclick: move |_| {
                onclose.call(());
            },
            div {
                class: "overlay-content {class}",
                onclick: move |e| {
                    e.stop_propagation();
                },

                div { class: "header",
                    div { class: "icon-holder", {icon} }
                    h3 { class: "title", {title.clone()} }
                    button {
                        class: "icon close-btn",
                        onclick: move |_| {
                            onclose.call(());
                        },
                        icons::X { width: "2.5rem", height: "2.5rem" }
                    }
                }
                div { class: "overlay-body", {children} }
            }
        }
    }
}
