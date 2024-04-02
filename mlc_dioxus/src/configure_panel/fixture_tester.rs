use dioxus::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::Message;

use mlc_common::patched::feature::{FeatureSetRequest, FixtureFeatureType};
use mlc_common::patched::PatchedFixture;

use crate::{icons, utils};
use crate::utils::Overlay;

#[component]
pub fn FixtureTester(info: PatchedFixture, onclose: EventHandler) -> Element {
    let features = info
        .features
        .iter()
        .map(|f| f.name())
        .collect::<Vec<_>>();
    let updater = use_coroutine(|mut rx: UnboundedReceiver<FeatureSetRequest>| {
        let fix_id = info.id;
        async move {
            let ws = utils::ws(&format!("/runtime/feature/{}", fix_id)).await;

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


    rsx! {
        Overlay{
            title: "Fixture Tester".to_owned(),
            class: "fixture-tester".to_owned(),
            onclose: move |_| {
                onclose.call(());
            },
            icon: rsx!(icons::Lamp {}),

            p {
                class: "info",
                "Name: ",
                span {
                    class: "name",
                    {info.name}
                },
                " Id: ",
                span {
                    class: "id",
                    {info.id.to_string()}
                }

            }

            div {
                class: "features",
                for feature in features {
                    match feature {
                        FixtureFeatureType::Dimmer => {rsx!{DimmerTester {
                            updater: updater,
                        }}}
                        FixtureFeatureType::White => {rsx!{WhiteTester{
                            updater: updater
                        }}}
                        FixtureFeatureType::Rgb => {rsx!{RgbTester{
                            updater: updater
                        }}}
                        FixtureFeatureType::Rotation => {rsx!{"Rotation"}}
                        FixtureFeatureType::PanTilt => {rsx!{PanTiltTester{
                            updater: updater
                        }}}
                        FixtureFeatureType::Amber => {rsx!{AmberTester{
                            updater: updater
                        }}}
                    }
                }
            }
        }
    }
}

#[component]
fn DimmerTester(updater: Coroutine<FeatureSetRequest>) -> Element {
    rsx! {
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
    }
}

#[component]
fn WhiteTester(updater: Coroutine<FeatureSetRequest>) -> Element {
    rsx! {
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
    }
}

#[component]
fn AmberTester(updater: Coroutine<FeatureSetRequest>) -> Element {
    rsx! {
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
    }
}

#[component]
fn RgbTester(updater: Coroutine<FeatureSetRequest>) -> Element {
    rsx! {
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
    }
}

#[component]
fn PanTiltTester(updater: Coroutine<FeatureSetRequest>) -> Element {
    rsx! {
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
    }
}
