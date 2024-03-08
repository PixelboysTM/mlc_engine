use dioxus::prelude::*;
use mlc_common::FixtureInfo;

#[derive(Props)]
pub struct FTProps<'a> {
    info: FixtureInfo,
    onclose: EventHandler<'a, ()>,
}

#[component]
pub fn FixtureTester<'a>(cx: Scope<'a, FTProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "overlay",
            onclick: move |_| {
              cx.props.onclose.call(());
            },
            div {
                class: "overlay-content fixture-tester",
                onclick: move |e| {
                  e.stop_propagation()
                },

                h3 {
                    "Fixture Tester",
                }
            }
        }
    })
}