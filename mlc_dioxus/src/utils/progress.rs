use dioxus::prelude::*;
use mlc_common::utils::{
    bounds::{One, Zero},
    BoundedValue,
};

#[component]
pub fn Progress(value: BoundedValue<f32, Zero, One>) -> Element {
    rsx! {
        div { class: "progress",
            span {
                class: "bar",
                style: format!("width: {}%", value.take() * 100.0)
            }
        }
    }
}
