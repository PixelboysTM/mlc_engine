use dioxus::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::Message;

use mlc_common::patched::feature::{FeatureSetRequest, FixtureFeatureType};
use mlc_common::patched::PatchedFixture;

use crate::{icons, utils};
use crate::utils::Overlay;

#[derive(Props)]
pub struct FTProps<'a> {
    info: PatchedFixture,
    onclose: EventHandler<'a, ()>,
}

#[component]
pub fn FixtureTester<'a>(cx: Scope<'a, FTProps<'a>>) -> Element<'a> {
    let features = cx
        .props
        .info
        .features
        .iter()
        .map(|f| f.name())
        .collect::<Vec<_>>();

    let create_eval = use_eval(cx);
    let updater = use_coroutine(cx, |mut rx: UnboundedReceiver<FeatureSetRequest>| {
        let eval = create_eval(r#"dioxus.send(window.location.host)"#).unwrap();
        let fix_id = cx.props.info.id;
        async move {
            let ws = utils::ws(&format!(
                "ws://{}/runtime/feature/{}",
                eval.recv()
                    .await
                    .map_err(|e| log::error!("Error: {e:?}"))
                    .unwrap()
                    .as_str()
                    .unwrap(),
                fix_id
            ));

            if let Ok(mut ws) = ws {
                while let Some(msg) = rx.next().await {
                    let s = ws
                        .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                        .await;
                    if s.is_err() {
                        log::error!("Error sending FeatureSetRequest: {}", s.err().unwrap());
                        break;
                    }
                }
            } else {
                log::error!("Failed to open feature websocket for id {}", fix_id);
            }
        }
    });


    cx.render(rsx! {
        Overlay{
            title: "Fixture Tester".to_owned(),
            class: "fixture-tester".to_owned(),
            onclose: move |_| {
                cx.props.onclose.call(());
            },
            icon: cx.render(rsx!(icons::Lamp {})),

            p {
                class: "info",
                "Name: ",
                span {
                    class: "name",
                    {cx.props.info.name.clone()}
                },
                " Id: ",
                span {
                    class: "id",
                    {cx.props.info.id.to_string()}
                }

            }

            div {
                class: "features",
                for feature in features {
                    match feature {
                        FixtureFeatureType::Dimmer => {cx.render(rsx!{DimmerTester {
                            updater: updater,
                        }})}
                        FixtureFeatureType::White => {cx.render(rsx!{WhiteTester{
                            updater: updater
                        }})}
                        FixtureFeatureType::Rgb => {cx.render(rsx!{RgbTester{
                            updater: updater
                        }})}
                        FixtureFeatureType::Rotation => {cx.render(rsx!{"Rotation"})}
                        FixtureFeatureType::PanTilt => {cx.render(rsx!{PanTiltTester{
                            updater: updater
                        }})}
                        FixtureFeatureType::Amber => {cx.render(rsx!{AmberTester{
                            updater: updater
                        }})}
                    }
                }
            }
        }
    })
}

#[component]
fn DimmerTester<'a>(cx: Scope<'a>, updater: &'a Coroutine<FeatureSetRequest>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "feature-tester dimmer",
            h3 {
                "Dimmer"
            }
            utils::Slider{
                initial: 0.0,
                onchange: move |v| {
                    updater.send(FeatureSetRequest::Dimmer {
                        value: v,
                    });
                }
            }
        }
    })
}

#[component]
fn WhiteTester<'a>(cx: Scope<'a>, updater: &'a Coroutine<FeatureSetRequest>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "feature-tester dimmer",
            h3 {
                "White"
            }
            utils::Slider{
                initial: 0.0,
                onchange: move |v| {
                    updater.send(FeatureSetRequest::White {
                        value: v,
                    });
                }
            }
        }
    })
}

#[component]
fn AmberTester<'a>(cx: Scope<'a>, updater: &'a Coroutine<FeatureSetRequest>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "feature-tester dimmer",
            h3 {
                "Amber"
            }
            utils::Slider{
                initial: 0.0,
                onchange: move |v| {
                    updater.send(FeatureSetRequest::White {
                        value: v,
                    });
                }
            }
        }
    })
}

#[component]
fn RgbTester<'a>(cx: Scope<'a>, updater: &'a Coroutine<FeatureSetRequest>) -> Element<'a> {
    cx.render(rsx! {
        div {
           class: "feature-tester rgb",
            h3 {
                "Rgb"
            },

            utils::RgbWidget{
                initial: (0.42,0.420,0.69),
                onchange: move |(r,g,b)| {
                    updater.send(FeatureSetRequest::Rgb {
                        red: r,
                        green: g,
                        blue: b,
                    });
                }
            }
        },
    })
}

#[component]
fn PanTiltTester<'a>(cx: Scope<'a>, updater: &'a Coroutine<FeatureSetRequest>) -> Element<'a> {
    cx.render(rsx! {
        div {
           class: "feature-tester pan-tilt",
            h3 {
                "Pan/Tilt"
            },

            utils::PanTiltWidget {
                initial: (0.5,0.5),
                onchange: move |(p,t)| {
                    updater.send(FeatureSetRequest::PanTilt {
                        pan: p,
                        tilt: t,
                    });
                }
            }
        },
    })
}
