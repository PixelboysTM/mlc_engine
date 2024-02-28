use std::ops::Deref;

use dioxus::prelude::*;
use reqwest::{IntoUrl, Url};
use wasm_logger::Config;

use crate::configure_panel::ConfigurePanel;
use crate::headbar::{Headbar, Pane};

mod headbar;
pub mod icons;
mod configure_panel;
mod utils;

fn main() {
    wasm_logger::init(Config::default());
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Pane::Configure);
    let pane = use_shared_state::<Pane>(cx).unwrap();

    cx.render(rsx! {
        Headbar{},
        div {
            width: "100vw",
            height: "calc(100vh - 3rem)",
            match pane.read().deref() {
                Pane::Configure => {
                    ConfigurePanel(cx)
                }
                Pane::Program => {
                    cx.render(rsx!{
                        "Program"
                    })
                }
                Pane::Show => {
                    cx.render(rsx!{
                        "Show"
                    })
                }
            }
        }
    })
}

async fn get_project_list() -> String {
    let d = gloo_net::http::Request::get(&build_url("/projects/projects-list"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    d
}

fn build_url(url: &str) -> String {
    let u = if cfg!(debug_assertions) {
        "https://localhost:8000"
    } else {
        ""
    };

    u.to_string() + url
}
