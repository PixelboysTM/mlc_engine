use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use mlc_common::effect::{D2RotationKey, D3PercentageKey, Key, PercentageKey, RotationKey};
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::utils;

#[component]
pub fn KeyEditor(px: f64, py: f64, children: Element, onclose: EventHandler) -> Element {
    rsx! {
        div {
            class: "key-edit",
            style: "--px: {px}px; --py: {py}px;",
            tabindex: -1,
            onmounted: move |e| {
                let _ = e.set_focus(true);
            },
            onfocusout: move |e| {
                let we = e.web_event();
                if let Some(current_target) = we
                    .current_target()
                    .map(|c| c.dyn_ref::<HtmlElement>().map(|c| c.clone()))
                    .flatten()
                {
                    if let Some(related_target) = we
                        .related_target()
                        .map(|c| c.dyn_ref::<HtmlElement>().map(|c| c.clone()))
                        .flatten()
                    {
                        let c = current_target.contains(Some(&related_target));
                        if c {
                            return;
                        }
                    }
                }
                onclose.call(());
            },
            onclick: move |e| {
                e.stop_propagation();
            },
            oncontextmenu: move |e| {
                e.stop_propagation();
            },
            {children}
        }
    }
}

pub trait DrawKeyWidget<T> {
    fn draw_widget<F>(&self, onchange: F) -> Element
    where
        F: FnMut(T) + 'static;
}

impl DrawKeyWidget<<PercentageKey as mlc_common::effect::Key>::Value> for PercentageKey {
    fn draw_widget<F>(&self, mut onchange: F) -> Element
    where
        F: FnMut(<PercentageKey as mlc_common::effect::Key>::Value) + 'static,
    {
        rsx! {
            div { style: "min-height: 12rem; min-width: 3rem",
                utils::Slider {
                    initial: self.value(),
                    onchange: move |v| {
                        onchange(v);
                    }
                }
            }
        }
    }
}

impl DrawKeyWidget<<D3PercentageKey as mlc_common::effect::Key>::Value> for D3PercentageKey {
    fn draw_widget<F>(&self, mut onchange: F) -> Element
    where
        F: FnMut(<D3PercentageKey as mlc_common::effect::Key>::Value) + 'static,
    {
        rsx! {
            div { style: "min-width: 12rem; min-height: 12rem;",
                utils::RgbWidget {
                    initial: self.value(),
                    onchange: move |v| {
                        onchange(v);
                    }
                }
            }
        }
    }
}

impl DrawKeyWidget<<D2RotationKey as mlc_common::effect::Key>::Value> for D2RotationKey {
    fn draw_widget<F>(&self, mut onchange: F) -> Element
    where
        F: FnMut(<D2RotationKey as mlc_common::effect::Key>::Value) + 'static,
    {
        rsx! {
            div { style: "min-width: 14rem; min-height: 14rem;",
                utils::PanTiltWidget {
                    initial: self.value(),
                    onchange: move |v| {
                        onchange(v);
                    }
                }
            }
        }
    }
}

impl DrawKeyWidget<<RotationKey as mlc_common::effect::Key>::Value> for RotationKey {
    fn draw_widget<F>(&self, mut onchange: F) -> Element
    where
        F: FnMut(<RotationKey as mlc_common::effect::Key>::Value) + 'static,
    {
        rsx! {"Unimplemented"}
    }
}
