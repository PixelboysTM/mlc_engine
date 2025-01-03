use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TabOrientation {
    #[default]
    Horizontal,
    Vertical,
    VerticalText,
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TabProps<I>
where
    I: Clone + PartialEq + 'static,
{
    orientation: Option<TabOrientation>,
    keys: Vec<I>,
    key_display: Callback<I, String>,
    content: Callback<I, Element>,
}

#[allow(non_snake_case)]
pub fn Tabs<I>(props: TabProps<I>) -> Element
where
    I: Clone + PartialEq + 'static,
{
    let mut current = use_signal(|| 0);
    let orient = props.orientation.unwrap_or_default();
    rsx! {
        div {
            class: "tabContainer",
            class: if orient == TabOrientation::Horizontal { "horizontal" },
            class: if orient == TabOrientation::Vertical { "vertical" },
            class: if orient == TabOrientation::VerticalText { "vertical v-text" },
            div { class: "tabs",
                for (i , tab) in props.keys.clone().into_iter().enumerate() {
                    button {
                        class: "tab",
                        class: if current() == i { "selected" },
                        onclick: move |_| {
                            current.set(i);
                        },
                        {props.key_display.call(tab)}
                    }
                }
            }
            div { class: "tabContent",
                if let Some(k) = props.keys.get(*current.read()) {
                    {props.content.call(k.clone())}
                }
            }
        }
    }
}
