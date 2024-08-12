use dioxus::prelude::*;
use futures::{select, SinkExt, StreamExt};
use gloo_net::websocket::Message;
use mlc_common::effect::{
    player::{EffectPlayerMsg, EffectPlayerRequest},
    EffectId,
};

use crate::utils::{self, Loading, ToWebSocketMessage};

#[component]
pub fn ShowPanel() -> Element {
    rsx! {
        div { class: "show-panel",
            div { class: "effect-player panel", EffectPlayer {} }
        }
    }
}

#[component]
fn EffectPlayer() -> Element {
    let all_effects = use_resource(move || async move {
        utils::fetch::<Vec<(String, EffectId)>>("/effects/get")
            .await
            .ok()
            .unwrap_or_else(Vec::new)
    });
    let mut playing_effects: Signal<Vec<EffectId>> = use_signal(Vec::new);
    let effect_player = use_coroutine(
        |mut rx: UnboundedReceiver<EffectPlayerRequest>| async move {
            let ws = utils::ws("/effects/effectPlayer").await;
            match ws {
                Ok(ws) => {
                    let mut ws = ws.fuse();
                    loop {
                        select! {
                            msg = rx.next() => {
                                if let Some(msg) = msg {
                                    let _ = ws.send(msg.to_msg().unwrap()).await;
                                }
                            }
                            msg = ws.next() => {
                                let m = match msg {
                                    Some(Ok(m)) => {
                                        match m {
                                            Message::Text(t) => serde_json::from_str::<EffectPlayerMsg>(&t).ok(),
                                            Message::Bytes(b) => serde_json::from_str::<EffectPlayerMsg>(&String::from_utf8(b).unwrap()).ok()
                                        }
                                    },
                                    Some(Err(e)) => {
                                        let e: gloo_net::websocket::WebSocketError = e;
                                        match e {
                                            gloo_net::websocket::WebSocketError::ConnectionClose(c) => {
                                                log::info!("WS was closed code: {}", c.code);
                                            }
                                            e => {
                                                log::error!("Websocket error: {e:?}");
                                            }
                                        }
                                        None
                                    }
                                    None => None,
                                };

                                match m {
                                    Some(msg) => {
                                        match msg {
                                            EffectPlayerMsg::PlayingEffects { effects } => playing_effects.set(effects),
                                        }
                                    }
                                    None => break,
                                }
                            }
                        }
                    }
                }
                Err(e) => log::error!("Failed to connect to effectPlayer: {e:?}"),
            }
        },
    );

    rsx! {
        match all_effects() {
            Some(effects) => {
                rsx!{
                    for e in effects {
                        div {
                            class: "effect",
                            p {
                                {e.0}
                            }
                        }
                    }
                }
            },
            None => rsx!{
                Loading {}
            },
        }
    }
}
