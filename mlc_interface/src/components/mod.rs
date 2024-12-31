use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};
use dioxus_toast::{ToastInfo, ToastManager};

mod modal;
pub use modal::*;

#[component]
pub fn IconButton<I: IconShape + Clone + PartialEq + 'static>(
    class: Option<String>,
    icon: I,
    onclick: Option<EventHandler<Event<MouseData>>>,
    popovertarget: Option<String>,
    popovertargetaction: Option<String>,
) -> Element {
    rsx! {
        button {
            class: "iBtn {class.clone().unwrap_or_default()}",
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
