use std::ops::Deref;

use dioxus::hooks::computed::use_tracked_state;
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use futures::{FutureExt, select, SinkExt, StreamExt};
use futures::future::{Either, select, Select};
use futures::lock::Mutex;
use futures::stream::{Next, SplitSink};
use gloo_net::websocket::{Message, WebSocketError};
use gloo_net::websocket::futures::WebSocket;

use mlc_common::{FaderUpdateRequest, ProjectDefinition, RuntimeUpdate, Settings};
use mlc_common::patched::{UniverseAddress, UniverseId};

use crate::utils;
use crate::utils::Loading;

#[component]
pub fn ConfigurePanel(cx: Scope) -> Element {
    let project_info = use_future(cx, (), |_| {
        utils::fetch::<ProjectDefinition>("/projects/current")
    });

    cx.render(rsx! {
        div {
            class: "configure-panel",
            div {
                class: "panel info",
                h3 {
                    class: "header",
                    "Project Info",
                },
                match project_info.value() {
                    Some(Ok(d)) => {cx.render(rsx!{
                        p {
                            span {
                                class: "pis",
                                "Name: "
                            },
                            {d.name.clone()}
                        },
                        p {
                            span {
                                class: "pis",
                                "Filename: "
                            },
                            {d.file_name.clone()}
                        },
                        p {
                            span {
                                class: "pis",
                                "Last Saved: "
                            },
                            {d.last_edited.format("%d.%m.%Y %H:%M:%S").to_string()}
                        }
                    })},
                    Some(Err(e)) => {cx.render(rsx!{"Error Loading Project Info"})},
                    None => {Loading(cx)}
                }
            },
            div {
                class: "panel fixture-types",
                utils::Loading {}
            },
            div {
                class: "panel universe-explorer",
                utils::Loading {}
            },
            div {
                class: "panel project-settings",
                ProjectSettings {}
            },
            div {
                class: "panel fader-browser",
                FaderPanel {}
            },
        }
    })
}

#[component]
fn ProjectSettings(cx: Scope) -> Element {
    let settings = use_future(cx, (), |_| utils::fetch::<Settings>("/settings/get"));
    let changed_settings = use_state(cx, || None);
    cx.render(rsx! {
        div {
            class: "project-settings-panel",
            h3 {
                class: "header",
                "Project Settings"
            },
            div {
                class: "settings",
                if changed_settings.get().is_some() {
                    cx.render(rsx!{
                        div {
                            class: "unsaved",
                            p {
                                "Unsaved changes press Update to Confirm."
                            }
                        }
                    })
                }

                match settings.value() {
                    Some(Ok(s)) => {
                        cx.render(
                            rsx!{
                                div {
                                    class: "setting",
                                    p {
                                        "Save on quit"
                                    },
                                    input {
                                        r#type: "checkbox",
                                        checked: {s.save_on_quit},
                                        onchange: move |v| {
                                            log::info!("{}", v.value);
                                            if let Some(ss) = changed_settings.get() {
                                                changed_settings.set(Some(Settings{
                                                save_on_quit: v.value == "true",
                                                ..*ss
                                                }))
                                            } else {
                                                changed_settings.set(Some(Settings{
                                                    save_on_quit: v.value == "true",
                                                        ..*s
                                                }))
                                            }
                                        }
                                    }
                                }
                            }
                        )
                    }
                    Some(Err(s)) => {cx.render(rsx!("Error Fetching settings"))}
                    None => {utils::Loading(cx)}
                }
            },
            div {
                class: "btns",
                button {
                    onclick: move |_| {
                        to_owned![changed_settings];
                        async move {
                            if let Some(s) = changed_settings.get() {
                                let s = utils::fetch_post::<String, _>("/settings/update", s.clone()).await;
                                if let Ok(_) = s {
                                    changed_settings.set(None);

                                } else {
                                    log::error!("{:?}", s);
                                }
                            }
                        }
                    },
                    "Update"
                }
            }
        }
    })
}


#[component]
fn FaderPanel(cx: Scope) -> Element {
    let current_universe = use_ref(cx, || 1);
    let universes = use_future(cx, (), |()| {
        to_owned![current_universe];
        async move {
            if let Ok(d) = utils::fetch::<Vec<u16>>("/data/universes").await {
                d
            } else {
                vec![]
            }
        }
    });
    let current_values = use_ref(cx, || [0_u8; 512]);

    let create_eval = use_eval(cx);


    let started = use_ref(cx, || false);
    let get = use_coroutine(cx, |mut rx: UnboundedReceiver<u16>| {
        let eval = create_eval(r#"dioxus.send(window.location.host)"#).unwrap();
        to_owned![current_values, current_universe, started];
        async move {
            if started.read().deref() == &true {
                return;
            }
            started.set(true);

            let ws_o = utils::ws(&format!(
                "ws://{}/runtime/fader-values/get",
                eval.recv()
                    .await
                    .map_err(|e| log::error!("Error"))
                    .unwrap()
                    .as_str()
                    .unwrap()
            ));

            if let Ok(mut get_ws) = ws_o {
                loop {
                    let i = select(rx.next(), get_ws.next()).await;
                    match i {
                        Either::Left((Some(msg), _)) => {
                            let _ = get_ws.send(Message::Text(msg.to_string())).await;
                        }
                        Either::Right((Some(Ok(msg)), _)) => {
                            let d = match msg {
                                Message::Text(t) => serde_json::from_str::<RuntimeUpdate>(&t),
                                Message::Bytes(b) => {
                                    serde_json::from_str::<RuntimeUpdate>(&String::from_utf8(b).unwrap())
                                }
                            };

                            if let Ok(update) = d {
                                match update {
                                    RuntimeUpdate::ValueUpdated {
                                        universe,
                                        channel_index,
                                        value,
                                    } => {
                                        if current_universe.read().deref() == &universe.0 {
                                            current_values.with_mut(|g| g[channel_index] = value);
                                        }
                                    }
                                    RuntimeUpdate::ValuesUpdated {
                                        universes,
                                        channel_indexes,
                                        values,
                                    } => {
                                        current_values.with_mut(|g| {
                                            for (i, index) in channel_indexes.iter().enumerate() {
                                                if current_universe.read().deref() == &universes[i].0 {
                                                    g[*index] = values[i];
                                                }
                                            }
                                        });
                                    }
                                    RuntimeUpdate::Universe {
                                        universe, values, ..
                                    } => {
                                        if current_universe.read().deref() == &universe.0 {
                                            current_values.set(values);
                                        }
                                    }
                                };
                            };
                        }

                        d => {
                            let b = match d {
                                Either::Left((a, b)) => format!("{:?}", a),
                                Either::Right((a, b)) => format!("{:?}", a)
                            };
                            log::error!("Error {b:?}");
                        }
                    };
                    async {}.await;
                }
            } else {
                log::error!("Error creating {:?}", ws_o.err().unwrap());
            }
        }
    });
    let set = use_coroutine(cx, |mut rx: UnboundedReceiver<FaderUpdateRequest>| {
        let eval = create_eval(r#"dioxus.send(window.location.host)"#).unwrap();

        async move {
            let ws = utils::ws(&format!(
                "ws://{}/runtime/fader-values/set",
                eval.recv()
                    .await
                    .map_err(|e| log::error!("Error"))
                    .unwrap()
                    .as_str()
                    .unwrap()
            ));

            if let Ok(mut ws) = ws {
                loop {
                    let m = rx.next().await;
                    if let Some(r) = m {
                        let r = ws.send(Message::Text(serde_json::to_string(&r).unwrap())).await;
                    }
                }
            } else {
                log::error!("Error opening websocket: {:?}", ws.err().unwrap());
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "slider-panel",
            div {
                class: "universe-list",
                match universes.value() {
                    Some(us) => {
                        cx.render(rsx!{
                            for u in us {
                    div {
                        class: "tab {sel(current_universe.read().deref() == u)}",
                        onclick: move |_| {
                            current_universe.set(*u);
                                        get.send(*u);
                        },
                        {u.to_string()}
                    }
                }
                        })
                    }
                    None => {Loading(cx)}
                },


            },
            div {
                class: "faders",
                (0..512).map(|i| {
                    rsx!{
                        Fader{
                        value: {current_values.read()[i]},
                        id: {i as u16},
                        onchange: move |v| {
                            set.send(FaderUpdateRequest{
                                universe: UniverseId(*current_universe.read().deref()),
                                channel: UniverseAddress::create(i as u16).expect("Must be"),
                                value: v
                            });
                        },
                    }
                    }
                })
            }
        }
    })
}

#[derive(Props)]
struct FaderProps<'a> {
    value: u8,
    id: u16,
    onchange: EventHandler<'a, u8>,
}

#[component]
fn Fader<'a>(cx: Scope<'a, FaderProps<'a>>) -> Element {
    let val = use_state(cx, || cx.props.value);
    let memo = use_memo(cx, &(cx.props.value, ), |(v, )| val.set(v));

    let size = use_state(cx, || 0.0);
    cx.render(rsx! {
        div {
            class: "fader-container",
            div {
               class: "name",
                {make_three_digit(cx.props.id)}
            },

            div{
                class: "range",
                background: "linear-gradient(0deg, var(--color-gradient-start) 0%, var(--color-gradient-end) {(*val.get() as f32 / 255.0) * 100.0}%, transparent {(*val.get() as f32 / 255.0) * 100.0}%, transparent 100%)",
                onmounted: move |e| {
                    log::info!("Val: {:?}", val.get());
                    to_owned![size];
                    async move {
                        let s = e.get_client_rect().await;
                        size.with_mut(|v| *v = s.unwrap().size.height);
                    }

                },

                onmousemove: move |e| {
                    if e.data.held_buttons() == MouseButton::Primary {
                        let p = e.data.element_coordinates();
                        let x = (1.0 - p.y / size.get()).min(1.0).max(0.0);
                        let v = (x * 255.0) as u8;
                        val.set(v);
                        cx.props.onchange.call(v);
                    }
                },
                onmousedown: move |e| {
                    if e.data.trigger_button() == Some(MouseButton::Primary) {
                        let p = e.data.element_coordinates();
                        let x = (1.0 - p.y / size.get()).min(1.0).max(0.0);
                        let v = (x * 255.0) as u8;
                        val.set(v);
                        cx.props.onchange.call(v);
                    }
                }

            },

            div{
                class: "value",
                {make_three_digit(*val.get() as u16)}
            }
        }
    })
}

fn make_three_digit(u: u16) -> String {
    format!("{:03}", u)
}

fn sel(b: bool) -> &'static str {
    if b {
        "sel"
    } else {
        ""
    }
}
