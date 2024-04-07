use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[component]
pub fn Popover(class: String, onclose: EventHandler, children: Element) -> Element {
    let mut pos = use_signal(|| (0.0, 0.0));

    rsx! {
        div {
            class: "popover-container",
            style: "--x: {pos().0}px; --y: {pos().1}px;",
            onmounted: move |e| async move{
                let r = e.get_client_rect().await;
                if let Ok(rect) = r {
                    pos.set((rect.origin.x, rect.origin.y))
                }
            },
            div {
                class: "popover-content",
                class: {class},
                tabindex: -1,
                onmounted: move |e| {
                    let _ = e.set_focus(true);
                },
                onfocusout: move |e| {
                    let we = e.web_event();
                    if let Some(current_target) = we.current_target().map(|c| c.dyn_ref::<HtmlElement>().map(|c| c.clone())).flatten() {
                        if let Some(related_target) = we.related_target().map(|c| c.dyn_ref::<HtmlElement>().map(|c| c.clone())).flatten() {
                            let c = current_target.contains(Some(&related_target));
                            if c {
                                return;
                            }
                        }
                    }
                    onclose.call(());
                },
                {children}
            }
        }
    }
}
