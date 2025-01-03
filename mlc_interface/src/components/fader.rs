use dioxus::prelude::*;

#[component]
pub fn Fader(value: MappedSignal<u8>, onchange: EventHandler<u8>) -> Element {
    rsx! {
        input {
            class: "fader",
            r#type: "range",
            min: 0,
            max: 255,
            value: value(),
            oninput: move |e| {
                let val = e.value().parse::<u8>().unwrap_or(0);
                onchange.call(val);
            },
        }
    }
}
