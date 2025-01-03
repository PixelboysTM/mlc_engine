use std::{rc::Rc, time::Duration};

use dioxus::{prelude::*, web::WebEventExt};
use dioxus_free_icons::{
    icons::ld_icons::{LdShieldAlert, LdShieldCheck, LdShieldOff},
    Icon, IconShape,
};
use dioxus_toast::{ToastInfo, ToastManager};

mod fader;
mod modal;
mod tabs;
pub use fader::*;
use mlc_common::Info;
pub use modal::*;
pub use tabs::*;
use web_sys::{wasm_bindgen::JsCast, HtmlDialogElement};

use crate::{
    log,
    utils::{fetch, reload_window},
};

#[component]
pub fn IconButton<I: IconShape + Clone + PartialEq + 'static>(
    class: Option<String>,
    icon: I,
    onclick: Option<EventHandler<Event<MouseData>>>,
    popovertarget: Option<String>,
    popovertargetaction: Option<String>,
    style: Option<String>,
) -> Element {
    rsx! {
        button {
            class: "iBtn {class.clone().unwrap_or_default()}",
            style,
            onclick: move |e| {
                if let Some(h) = &onclick {
                    h.call(e);
                }
            },
            popovertarget,
            popovertargetaction,
            Icon { icon }
        }
    }
}

#[component]
pub fn Spinner(class: Option<String>) -> Element {
    rsx! {
        div { class: format!("cmpSpinner {}", class.unwrap_or_default()) }
    }
}

#[allow(dead_code)]
pub trait ToastAdditions {
    fn success<T1: AsRef<str>, T2: AsRef<str>>(&mut self, heading: T1, message: T2);
    fn warn<T1: AsRef<str>, T2: AsRef<str>>(&mut self, heading: T1, message: T2);
    fn error<T1: AsRef<str>, T2: AsRef<str>>(&mut self, heading: T1, message: T2);
    fn info<T1: AsRef<str>, T2: AsRef<str>>(&mut self, heading: T1, message: T2);
}

impl ToastAdditions for Signal<ToastManager> {
    fn success<T1: AsRef<str>, T2: AsRef<str>>(&mut self, heading: T1, message: T2) {
        self.write().popup(ToastInfo {
            allow_toast_close: true,
            context: message.as_ref().to_string(),
            heading: Some(heading.as_ref().to_string()),
            hide_after: Some(10),
            position: dioxus_toast::Position::BottomRight,
            icon: Some(dioxus_toast::Icon::Success),
        });
    }

    fn warn<T1: AsRef<str>, T2: AsRef<str>>(&mut self, heading: T1, message: T2) {
        self.write().popup(ToastInfo {
            allow_toast_close: true,
            context: message.as_ref().to_string(),
            heading: Some(heading.as_ref().to_string()),
            hide_after: Some(10),
            position: dioxus_toast::Position::BottomRight,
            icon: Some(dioxus_toast::Icon::Warning),
        });
    }

    fn error<T1: AsRef<str>, T2: AsRef<str>>(&mut self, heading: T1, message: T2) {
        self.write().popup(ToastInfo {
            allow_toast_close: true,
            context: message.as_ref().to_string(),
            heading: Some(heading.as_ref().to_string()),
            hide_after: Some(10),
            position: dioxus_toast::Position::BottomRight,
            icon: Some(dioxus_toast::Icon::Error),
        });
    }

    fn info<T1: AsRef<str>, T2: AsRef<str>>(&mut self, heading: T1, message: T2) {
        self.write().popup(ToastInfo {
            allow_toast_close: true,
            context: message.as_ref().to_string(),
            heading: Some(heading.as_ref().to_string()),
            hide_after: Some(10),
            position: dioxus_toast::Position::BottomRight,
            icon: Some(dioxus_toast::Icon::Info),
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ConnectionStatus {
    Healthy,
    Hickup,
    Lost,
}

pub static LATENCY: GlobalSignal<u64> = Signal::global(|| 0);

#[component]
pub fn DissconnetHelper() -> Element {
    let mut status = use_signal(|| ConnectionStatus::Healthy);
    let mut ele: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_future(move || async move {
        let mut failed: u32 = 0;
        loop {
            let timer = wasm_timer::Instant::now();
            let r = fetch::<String>("/util/heartbeat").await;
            let time = timer.elapsed().as_millis() as u64;
            *LATENCY.write() = time;
            if r.is_ok() {
                failed = 0;
            } else {
                failed += 1;
                log!("Failed a heartbeat.");
            }

            match failed {
                0 => status.set(ConnectionStatus::Healthy),
                1..=6 => status.set(ConnectionStatus::Hickup),
                7 => {
                    status.set(ConnectionStatus::Lost);
                    // Show modal
                    if let Some(ele) = ele.read().clone() {
                        let ev: HtmlDialogElement =
                            ele.as_web_event().dyn_into().expect("It is this");
                        let _ = ev.show_modal();
                    }
                }
                8.. => status.set(ConnectionStatus::Lost),
            }

            gloo::timers::future::sleep(Duration::from_secs(1)).await;
        }
    });

    let info: Signal<Info> = use_context();
    use_effect(move || {
        if *info.read() == Info::SystemShutdown {
            if let Some(ele) = ele.read().clone() {
                let ev: HtmlDialogElement = ele.as_web_event().dyn_into().expect("It is this");
                let _ = ev.show_modal();
            }
        }
    });

    rsx! {
        div { class: "dissconnectHelper",
            match status() {
                ConnectionStatus::Healthy => rsx! {
                    Icon { icon: LdShieldCheck, style: "color: var(--c-info);" }
                },
                ConnectionStatus::Hickup => rsx! {
                    Icon { icon: LdShieldAlert, style: "color: var(--c-warn);" }
                },
                ConnectionStatus::Lost => rsx! {
                    Icon { icon: LdShieldOff, style: "color: var(--c-err);" }
                },
            }
        }

        dialog {
            class: "dissconnectModal",
            onmounted: move |e| {
                ele.set(Some(e.data()));
            },
            h1 { "Dissconnected" }
            p {
                match status() {
                    ConnectionStatus::Healthy => {
                        "Connection retrieved! A full reload is needed to make sure everything is in sync."
                    }
                    _ => {
                        "Connection to the backend is brocken, maybe it was turned of or is unreachable? Trying to reconnect!"
                    }
                }
            }

            match status() {
                ConnectionStatus::Healthy => rsx! {
                    button {
                        onclick: move |_| {
                            let _ = reload_window();
                        },
                        "Reload"
                    }
                },
                _ => rsx! {
                    Spinner {}
                },
            }
        }
    }
}

#[component]
pub fn Panel(
    children: Element,
    pos_x: (u8, u8),
    pos_y: (u8, u8),
    ident: String,
    title: Option<String>,
) -> Element {
    rsx! {
        div {
            class: "panel-c {ident}",
            style: "grid-column-start: {pos_x.0}; grid-column-end: {pos_x.1}; grid-row-start: {pos_y.0}; grid-row-end: {pos_y.1};",
            if let Some(title) = title {
                h1 { {title} }
            }
            div { class: "panel {ident}",
                SuspenseBoundary {
                    fallback: move |_context: SuspenseContext| rsx! {
                        Spinner {}
                    },
                    ErrorBoundary {
                        handle_error: move |context: ErrorContext| {
                            for error in context.errors().iter() {
                                gloo::console::error!(format!("{error:?}"));
                            }
                            rsx! { "Failed" }
                        },

                        {children}
                    }
                }
            }
        }
    }
}
