use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use gloo_storage::Storage;

use mlc_common::effect::Effect;
use mlc_common::Info;
use mlc_common::uuid::Uuid;

use crate::{icons, utils};

#[component]
pub fn ProgramPanel() -> Element {
    let current_effect = use_context_provider::<Signal<Option<Effect>>>(|| Signal::new(None));

    let mut effect_browser_out = use_signal(|| true);

    rsx! {
        div {
            class: "program-panel",
            class: if !effect_browser_out() {"no-browser"},
            if effect_browser_out() {
                div {
                    class: "panel effect-browser",
                    h3 {
                        class: "header",
                        "Effect Browser",
                        button {
                            class: "icon close-browser-btn",
                            onclick: move |_| {
                              effect_browser_out.set(false);
                            },
                            icons::PanelLeftClose{},
                        },
                    },
                    EffectBrowser {}
                }
            }

            if !effect_browser_out() {
                div {
                    class: "effect-browser-open-btn",
                    onclick: move |_| {
                        effect_browser_out.set(true);
                    },
                    icons::PanelLeftOpen {}
                }
            }

            div {
                class: "panel effect-info",
                "Effect Info",
                {format!("Effect: {:?}", current_effect())}
            },
            div {
                class: "panel timeline",
                "Timeline"
            },
            div {
                class: "panel visualizer",
                "Visualizer"
            }
        }
    }
}

#[component]
fn EffectBrowser() -> Element {
    let mut effect_list = use_resource(|| async {
        utils::fetch::<Vec<(String, Uuid)>>("/effects/get").await.map(|effects| {
            build_effect_tree(&effects)
        }).map_err(|e| {
            log::error!("{e:?}");
        })
    });

    let browser_register: Signal<HashMap<String, bool>> = use_signal(||
        gloo_storage::SessionStorage::get::<HashMap<String, bool>>("effectBrowserOpenMap").unwrap_or(HashMap::new())
    );

    use_effect(move || {
        gloo_storage::SessionStorage::set("effectBrowserOpenMap", browser_register()).expect("");
    });

    let info = use_context::<Signal<Info>>();

    use_effect(move || {
        if info() == Info::EffectListChanged {
            effect_list.restart();
        }
    });

    match &*effect_list.read_unchecked() {
        Some(Ok(effects)) => {
            rsx! {
                DrawEffectTree {
                    tree: effects.clone(),
                    browser_register,
                }
            }
        }
        Some(Err(_)) => {
            rsx! {
                "Error loading effect library",
            }
        }
        None => {
            rsx! {
            utils::Loading {}
        }
        }
    }
}

#[component]
fn DrawEffectTree(tree: Vec<Rc<RefCell<Tree>>>, browser_register: Signal<HashMap<String, bool>>) -> Element {
    let elements = tree.iter().map(|e| e.borrow().clone()).collect::<Vec<_>>();

    // let mut browser_register: Signal<HashMap<String, bool>> = use_context();

    rsx! {
        div {
            class: "effect-tree",
            for i in elements {
                match i.data {
                    TreeItem::Effect{ label, .. } => {
                        rsx! {
                            div {
                                class: "element effect",
                                ondoubleclick: move |e| {
                                    log::info!("{e:?}");
                                },
                                icons::Sparkles {
                                    width: "1rem",
                                    height: "1rem",
                                },
                                {label.clone()}
                            }
                        }
                    }
                    TreeItem::Folder{ name, path } => {
                        rsx! {
                            div {
                                class: "element folder",
                                onclick: move |_| {
                                    let p = path.clone();
                                    let mut w = browser_register.write();
                                    let v = w.get(&p as &str);
                                    let new_val = if let Some(val) = v {
                                        !val
                                    } else {
                                        false
                                    };

                                    w.insert(path.clone(), new_val);
                                },
                                icons::Folder {
                                    width: "1rem",
                                    height: "1rem",
                                },
                                {name.clone()},
                            },
                            if *browser_register().get(&path as &str).unwrap_or(&true) {
                            div {
                                    class: "children",
                                    DrawEffectTree {
                                        tree: i.children.clone(),
                                        browser_register,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum TreeItem {
    Effect {
        name: String,
        label: String,
        id: Uuid,
    },
    Folder {
        path: String,
        name: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tree {
    data: TreeItem,
    children: Vec<Rc<RefCell<Tree>>>,
}

fn build_effect_tree(effects: &[(String, Uuid)]) -> Vec<Rc<RefCell<Tree>>> {
    let mut trees: Vec<Rc<RefCell<Tree>>> = vec![];

    for (raw_name, id) in effects {
        let split = raw_name.split('/').collect::<Vec<&str>>();
        let split_ref: &[&str] = &split;

        let (path, name) = match split_ref {
            [n] => (vec![], *n),
            [p @ .., n] => (p.iter().cloned().collect::<Vec<_>>(), *n),
            [] => unreachable!("Why does split return an empty list!")
        };

        fn create_effect(raw: String, name: String, id: Uuid) -> Rc<RefCell<Tree>> {
            Rc::new(RefCell::new(Tree {
                data: TreeItem::Effect {
                    name: raw,
                    label: name,
                    id,
                },
                children: vec![],
            }))
        }

        let new_effect = create_effect(raw_name.to_string(), name.to_string(), id.clone());

        if path.is_empty() {
            trees.push(new_effect);
        } else {
            let parent = find_parent(&mut trees, &path, "");
            parent.borrow_mut().children.push(new_effect);
        }
    }
    trees
}

fn find_parent(children: &mut Vec<Rc<RefCell<Tree>>>, paths: &[&str], full_path: &str) -> Rc<RefCell<Tree>> {
    let (path, rest) = match paths {
        [path, rest @ ..] => (path, rest),
        [] => unreachable!()
    };

    let p = children.iter().find(|e| match &e.borrow().data {
        TreeItem::Effect { .. } => { false }
        TreeItem::Folder { name, .. } => { name == path }
    }).cloned();

    let parent = if let Some(pr) = p {
        pr
    } else {
        let pr = Rc::new(RefCell::new(Tree {
            data: TreeItem::Folder {
                name: path.to_string(),
                path: format!("{}/{}", full_path, path),
            },
            children: vec![],
        }));
        children.push(pr.clone());
        pr
    };

    if rest.is_empty() {
        parent
    } else {
        find_parent(&mut parent.borrow_mut().children, rest, &format!("{}/{}", full_path, path))
    }
}