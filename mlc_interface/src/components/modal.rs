use crate::IconButton;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdX, Icon, IconShape};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalMode {
    Auto,
    Manual,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalLayout {
    Confirm,
    ConfirmCancel,
}

#[component]
pub fn Modal<
    I: IconShape + Clone + PartialEq + 'static,
    S: AsRef<str> + Clone + PartialEq + 'static,
>(
    children: Element,
    id: String,
    mode: Option<ModalMode>,
    layout: Option<ModalLayout>,
    heading: S,
    icon: I,
    onconfirm: Option<EventHandler>,
    oncancel: Option<EventHandler>,
) -> Element {
    rsx! {
        div {
            class: "dialog",
            popover: match mode.unwrap_or_else(|| ModalMode::Auto) {
                ModalMode::Auto => "auto",
                ModalMode::Manual => "manual",
            },
            id: id.clone(),
            div { class: "heading",
                Icon { icon, class: "icon" }
                h2 { {heading.as_ref().to_string()} }
                IconButton {
                    icon: LdX,
                    class: "close",
                    popovertarget: id.clone(),
                    popovertargetaction: "hide",
                }
            }
            div { class: "content", {children} }
            div { class: "footer",
                match layout.unwrap_or_else(|| ModalLayout::ConfirmCancel) {
                    ModalLayout::Confirm => rsx! {
                        button {
                            popovertarget: id.clone(),
                            popovertargetaction: "hide",
                            onclick: move |_| {
                                if let Some(e) = onconfirm {
                                    e.call(());
                                }
                            },
                            "Okay"
                        }
                    },
                    ModalLayout::ConfirmCancel => rsx! {
                        button {
                            popovertarget: id.clone(),
                            popovertargetaction: "hide",
                            onclick: move |_| {
                                if let Some(e) = onconfirm {
                                    e.call(());
                                }
                            },
                            "Okay"
                        }
                        button {
                            popovertarget: id.clone(),
                            popovertargetaction: "hide",
                            onclick: move |_| {
                                if let Some(e) = oncancel {
                                    e.call(());
                                }
                            },
                            "Cancel"
                        }
                    },
                }
            }
        }
    }
}
