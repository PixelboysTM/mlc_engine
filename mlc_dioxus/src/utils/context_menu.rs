use std::rc::Rc;
use std::sync::Mutex;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    x_pos: f64,
    y_pos: f64,
}

impl ContextMenu {
    pub fn new(x_pos: f64, y_pos: f64) -> Self {
        Self {
            items: vec![],
            x_pos,
            y_pos,
        }
    }

    pub fn add<F>(mut self, name: impl Into<String>, action: F) -> Self where F: FnMut(Event<MouseData>) -> bool + 'static {
        self.items.push(ContextMenuItem {
            name: name.into(),
            action: Rc::new(Mutex::new(action)),
        });
        self
    }
}

#[derive(Clone)]
pub struct ContextMenuItem {
    name: String,
    action: Rc<Mutex<dyn FnMut(Event<MouseData>) -> bool + 'static>>,
}

impl PartialEq for ContextMenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[component]
pub fn ContextMenu(mut menu: ContextMenu, onclose: EventHandler) -> Element {
    let child_count = menu.items.len();
    rsx! {
            div {
                class: "context-menu",
                style: "--cm-p-x: {menu.x_pos}px; --cm-p-y: {menu.y_pos}px; --cm-cc: {child_count};",
                tabindex: -1,
                onmounted: move |e| {
                    let _ = e.set_focus(true);

                },
                onclick: move |e| {
                    e.stop_propagation();
                },
                onfocusout: move |_| {
                    onclose.call(());
                },
                for item in menu.items {
                    div {
                        class: "menu-item",
                        onclick: move |e| {
                            if item.action.lock().expect("")(e) {
                                onclose.call(());
                            }
                        },
                        p {
                            {item.name.clone()}
                        }
                    }
                }
            }

    }
}