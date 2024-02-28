use dioxus::prelude::*;
use gloo_net::websocket::futures::WebSocket;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsError, JsValue};

pub async fn fetch<T>(url: &str) -> Result<T, gloo_net::Error>
where
    T: DeserializeOwned,
{
    gloo_net::http::Request::get(url)
        .send()
        .await?
        .json::<T>()
        .await
}

pub async fn fetch_post<T, B>(url: &str, body: B) -> Result<T, gloo_net::Error>
where
    T: DeserializeOwned,
    B: Serialize,
{
    gloo_net::http::Request::post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body)?)?
        .send()
        .await?
        .json::<T>()
        .await
}

pub fn ws(url: &str) -> Result<WebSocket, gloo_utils::errors::JsError> {
    WebSocket::open(url)
}

#[component]
pub fn Loading(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "loading-spinner",
            div {
                class: "inner",
            }
        }
    })
}
