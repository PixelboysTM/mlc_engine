use std::fmt::{Debug, Display};

use dioxus::prelude::*;
use dioxus_toast::ToastManager;
use gloo::net::websocket::futures::WebSocket;
use serde::{de::DeserializeOwned, Serialize};

use crate::components::ToastAdditions;

#[macro_use]
mod console {
    #[macro_export]
    macro_rules! log {
        ($($arg:expr),+) => {
            gloo::console::log!("LOG", $($arg),+)
        };
    }
}

pub async fn fetch<T>(url: &str) -> Result<T, gloo::net::Error>
where
    T: DeserializeOwned,
{
    gloo::net::http::Request::get(url)
        .send()
        .await?
        .json::<T>()
        .await
}

pub async fn post<T, B>(url: &str, body: B) -> Result<T, gloo::net::Error>
where
    T: DeserializeOwned,
    B: Serialize,
{
    gloo::net::http::Request::post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body)?)?
        .send()
        .await?
        .json::<T>()
        .await
}

pub async fn ws(url: &str) -> Result<WebSocket, String> {
    let host = gloo::utils::window().location().host().to_err()?;
    WebSocket::open(&format!("ws://{host}{url}")).to_err()
}

pub fn subscribe_ws<T: DeserializeOwned + 'static>(url: &str, handler: EventHandler<T>) {
    use futures::StreamExt;
    let url = url.to_string();
    use_future(move || {
        let url = url.clone();
        async move {
            let ws = match ws(&url).await.to_err() {
                Ok(w) => w,
                Err(e) => {
                    log!(e);
                    return;
                }
            };
            let mut ws = ws;
            while let Some(Ok(msg)) = ws.next().await {
                let msg = match msg {
                    gloo::net::websocket::Message::Text(t) => t,
                    gloo::net::websocket::Message::Bytes(vec) => {
                        String::from_utf8(vec).expect("No valid Json encoded data")
                    }
                };
                let data = serde_json::from_str::<T>(&msg).expect("No valid Json");
                handler.call(data);
            }
        }
    });
}

pub fn reload_window() -> Result<(), String> {
    gloo::utils::window().location().reload().to_err()
}

pub trait ToErrString {
    type R;
    fn to_err(self) -> Result<Self::R, String>;
}

impl<V, E: Debug> ToErrString for Result<V, E> {
    type R = V;

    fn to_err(self) -> Result<Self::R, String> {
        self.map_err(|e| format!("{e:?}"))
    }
}

pub trait ToErrToast {
    type D;
    fn catch_toast<S1: AsRef<str>, S2: Display>(
        self,
        toast: &mut Signal<ToastManager>,
        heading: S1,
        msg: S2,
        default: Self::D,
    ) -> Self::D;
}

impl<V, E: Debug> ToErrToast for Result<V, E> {
    type D = V;

    fn catch_toast<S1: AsRef<str>, S2: Display>(
        self,
        toast: &mut Signal<ToastManager>,
        heading: S1,
        msg: S2,
        default: Self::D,
    ) -> Self::D {
        self.map_err(|e| toast.error(heading, format!("{msg}{e:?}")))
            .unwrap_or(default)
    }
}
