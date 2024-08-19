use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use chrono::Duration;
use dioxus::prelude::*;
use futures::{select, SinkExt, StreamExt};
use gloo_net::websocket::Message;
use gloo_storage::Storage;
use key_editor::KeyFrameInspector;
use log::{info, warn};

use mlc_common::effect::rest::{EffectHandlerRequest, EffectHandlerResponse};
use mlc_common::effect::Effect;
use mlc_common::utils::FormatEffectDuration;
use mlc_common::uuid::Uuid;
use mlc_common::Info;

use crate::program_panel::effect_timeline::EffectTimeline;
use crate::utils::popover::Popover;
use crate::utils::toaster::{Toaster, ToasterWriter};
use crate::utils::ToWebSocketMessage;
use crate::{icons, utils};

mod effect_timeline;
mod key_editor;

#[derive(Debug, PartialEq)]
struct EffectInvalidate;

#[component]
pub fn ProgramPanel() -> Element {
    let mut current_effect = use_context_provider::<Signal<Option<Effect>>>(|| Signal::new(None));
    let _current_keyframe =
        use_context_provider::<Signal<Option<(usize, usize)>>>(|| Signal::new(None));

    let mut toaster = use_context::<Signal<Toaster>>();

    let effect_handler = use_coroutine(|mut rx: UnboundedReceiver<EHRequest>| async move {
        let ws = utils::ws("/effects/effectHandler").await;
        match ws {
            Ok(ws) => {
                let mut ws = ws.fuse();
                loop {
                    select! {
                        msg = rx.next() => {
                            if let Some(msg) = msg {
                                let msg: EHRequest = msg;
                                match msg {
                                    EHRequest::Open(id) => {
                                        let _ = ws.send(EffectHandlerRequest::Get {
                                            id,
                                        }.to_msg().unwrap()).await;
                                    }
                                    EHRequest::Update(effect) => {
                                        let _ = ws.send(EffectHandlerRequest::Update {
                                            id: effect.id,
                                            looping: effect.looping,
                                            duration: effect.duration,
                                            tracks: effect.tracks,
                                        }.to_msg().unwrap()).await;
                                    }
                                    EHRequest::Create(name) => {
                                        let _ = ws.send(EffectHandlerRequest::Create {
                                            name,
                                        }.to_msg().unwrap()).await;
                                    }
                                }
                            }
                        },
                        msg = ws.next() => {
                            let d = match msg {
                                Some(Ok(msg)) => {
                                    match msg {
                                        Message::Text(t) => serde_json::from_str::<EffectHandlerResponse>(&t).ok(),
                                        Message::Bytes(b) => serde_json::from_str::<EffectHandlerResponse>(
                                            &String::from_utf8(b).unwrap(),
                                        ).ok()
                                    }
                                }
                                Some(Err(e)) => {
                                    let e: gloo_net::websocket::WebSocketError = e;
                                    match e {
                                        gloo_net::websocket::WebSocketError::ConnectionClose(c) => {
                                            log::info!("WS was closed code: {}", c.code);
                                        },
                                        e => {
                                            log::error!("Websocket error: {e:?}");
                                        },
                                    }
                                    None
                                }
                                None => {
                                    None
                                }
                            };

                            if let Some(update) = d {
                                match update {
                                    EffectHandlerResponse::EffectCreated{ name, .. } => {
                                        toaster.info("Created Effect", &format!("Created Effect: {name}"));
                                        info!("update: EffectCreated");
                                    }
                                    EffectHandlerResponse::EffectUpdated{ id } => {
                                        info!("update: EffectUpdated");
                                        if Some(id) == current_effect().map(|e| e.id) {
                                            let _ = ws.send(EffectHandlerRequest::Get {
                                                id,
                                            }.to_msg().unwrap()).await;
                                        }
                                    }
                                    EffectHandlerResponse::EffectRunning{ .. } => {}
                                    EffectHandlerResponse::EffectList{ .. } => {
                                        log::warn!("Received effect list via ws why do we use this?")
                                    }
                                    EffectHandlerResponse::Effect{ effect } => {
                                        current_effect.set(Some(effect));
                                        info!("update: Effect");
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Unable to open ws to effectHandler: {e:?}");
                toaster.error(
                    "Effect Handler error!",
                    "Unable to open effectHandler see console for more detailed information.",
                );
            }
        }
    });

    let _effect_invalidator = use_coroutine(
        move |mut rx: UnboundedReceiver<EffectInvalidate>| async move {
            while (rx.next().await).is_some() {
                if let Some(e) = &*current_effect.peek() {
                    effect_handler.send(EHRequest::Update(e.clone()));
                } else {
                    warn!("Why needs update when no effect is loaded?");
                }
            }
        },
    );

    let mut effect_browser_out = use_signal(|| true);

    rsx! {
        div {
            class: "program-panel",
            class: if !effect_browser_out() { "no-browser" },
            if effect_browser_out() {
                div { class: "panel effect-browser",
                    h3 { class: "header",
                        "Effect Browser"
                        button {
                            class: "icon close-browser-btn",
                            onclick: move |_| {
                                effect_browser_out.set(false);
                            },
                            icons::PanelLeftClose {}
                        }
                    }
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

            div { class: "panel effect-info",
                h3 { class: "header", "Effect Info" }
                EffectInfo {}
            }
            div { class: "panel timeline", EffectTimeline {} }
            div { class: "panel inspector", KeyFrameInspector {} }
        }
    }
}

#[derive(Debug, Clone)]
enum EHRequest {
    Open(Uuid),
    Update(Effect),
    Create(String),
}

#[component]
fn EffectBrowser() -> Element {
    let mut effect_list = use_resource(|| async {
        utils::fetch::<Vec<(String, Uuid)>>("/effects/get")
            .await
            .map(|effects| build_effect_tree(&effects))
            .map_err(|e| {
                log::error!("{e:?}");
            })
    });

    let browser_register: Signal<HashMap<String, bool>> = use_signal(|| {
        gloo_storage::SessionStorage::get::<HashMap<String, bool>>("effectBrowserOpenMap")
            .unwrap_or(HashMap::new())
    });

    use_effect(move || {
        gloo_storage::SessionStorage::set("effectBrowserOpenMap", browser_register()).expect("");
    });

    let effect_handler: Coroutine<EHRequest> = use_coroutine_handle();

    let info = use_context::<Signal<Info>>();

    use_effect(move || {
        if info() == Info::EffectListChanged {
            effect_list.restart();
        }
    });

    let mut new_effect = use_signal(|| false);

    match &*effect_list.read_unchecked() {
        Some(Ok(effects)) => {
            rsx! {
                DrawEffectTree {
                    tree: effects.clone(),
                    browser_register,
                    on_open_effect: move |id| {
                        effect_handler.send(EHRequest::Open(id));
                    }
                }
                button {
                    class: "icon create-effect",
                    title: "Create New Effect",
                    onclick: move |_| {
                        new_effect.set(true);
                    },
                    icons::Plus {}
                }

                if new_effect() {
                    utils::Overlay {
                        title: "Create new Effect",
                        class: "create-effect-overlay",
                        icon: rsx! {
                            icons::Sparkles {}
                        },
                        onclose: move |_| {
                            new_effect.set(false);
                        },
                        input {
                            r#type: "text",
                            value: "New Effect",
                            onchange: move |e| {
                                let v = e.value();
                                let name = v.trim();
                                if !name.is_empty() {
                                    effect_handler.send(EHRequest::Create(name.to_string()));
                                    new_effect.set(false);
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(Err(_)) => {
            rsx! { "Error loading effect library" }
        }
        None => {
            rsx! {
                utils::Loading {}
            }
        }
    }
}

#[component]
fn DrawEffectTree(
    tree: Vec<Rc<RefCell<Tree>>>,
    on_open_effect: EventHandler<Uuid>,
    browser_register: Signal<HashMap<String, bool>>,
) -> Element {
    let elements = tree.iter().map(|e| e.borrow().clone()).collect::<Vec<_>>();

    rsx! {
        div { class: "effect-tree",
            for i in elements {
                match i.data {
                    TreeItem::Effect{ label, id, .. } => {
                        rsx! {
                            div {
                                class: "element effect",
                                ondoubleclick: move |_| {
                                    on_open_effect.call(id);
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

                                match *browser_register().get(&path as &str).unwrap_or(&true) {
                                    true => {
                                        rsx! {
                                            icons::FolderOpen {
                                                width: "1rem",
                                                height: "1rem",
                                            },
                                        }
                                    }
                                    false => {
                                        rsx! {
                                            icons::Folder {
                                                width: "1rem",
                                                height: "1rem",
                                            },
                                        }
                                    }
                                }
                                {name.clone()},
                            },
                            if *browser_register().get(&path as &str).unwrap_or(&true) {
                            div {
                                    class: "children",
                                    DrawEffectTree {
                                        tree: i.children.clone(),
                                        browser_register,
                                        on_open_effect,
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
            [p @ .., n] => (p.to_vec(), *n),
            [] => unreachable!("Why does split return an empty list!"),
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

        let new_effect = create_effect(raw_name.to_string(), name.to_string(), *id);

        if path.is_empty() {
            trees.push(new_effect);
        } else {
            let parent = find_parent(&mut trees, &path, "");
            parent.borrow_mut().children.push(new_effect);
        }
    }
    trees
}

fn find_parent(
    children: &mut Vec<Rc<RefCell<Tree>>>,
    paths: &[&str],
    full_path: &str,
) -> Rc<RefCell<Tree>> {
    let (path, rest) = match paths {
        [path, rest @ ..] => (path, rest),
        [] => unreachable!(),
    };

    let p = children
        .iter()
        .find(|e| match &e.borrow().data {
            TreeItem::Effect { .. } => false,
            TreeItem::Folder { name, .. } => name == path,
        })
        .cloned();

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
        find_parent(
            &mut parent.borrow_mut().children,
            rest,
            &format!("{}/{}", full_path, path),
        )
    }
}

#[component]
fn EffectInfo() -> Element {
    let mut current_effect = use_context::<Signal<Option<Effect>>>();
    let effect_invalidator: Coroutine<EffectInvalidate> = use_coroutine_handle();

    let mut edit_duration = use_signal(|| false);

    rsx! {
        match current_effect() {
            None => {
                rsx! {"No Effect currently loaded!"}
            }
            Some(effect) => {
                rsx!{
                    div {
                        class: "property-container",
                        div {
                            class: "property",
                            p {
                                "Name",
                            },
                            p {
                                to_visualized_effect_name {
                                    name: effect.name,
                                }
                            }
                        }
                        div {
                            class: "property",
                            p {
                                "Looping",
                            },
                            utils::Toggle {
                                value: effect.looping,
                                onchange: move |v| {
                                    {
                                        let mut w = current_effect.write();
                                        if let Some(w) = &mut *w {
                                            w.looping = v;
                                        }
                                    }
                                    effect_invalidator.send(EffectInvalidate);
                                }
                            },
                        },
                        div {
                            class: "property",
                            p {
                                "Duration"
                            },
                            p {
                                class: "effect-duration",
                                onclick: move |_| {
                                    edit_duration.set(true);
                                },
                                {effect.duration.effect_format()},
                                if edit_duration() {
                                    Popover {
                                        class: "edit-effect-duration",
                                        onclose: move |_| {
                                            edit_duration.set(false);
                                        },
                                        input {
                                            class: "minutes",
                                            r#type: "number",
                                            min: 0,
                                            value: effect.duration.num_minutes(),
                                            onchange: move |v| {
                                                let minutes = effect.duration.num_minutes();
                                                let new_minutes = v.value().parse::<i64>().unwrap_or(0);
                                                let delta = new_minutes - minutes;
                                                log::info!("Delta: {delta}");
                                                {
                                                    let mut w = current_effect.write();
                                                    if let Some(w) = &mut *w {
                                                        w.duration += Duration::minutes(delta);
                                                        w.duration = w.duration.max(Duration::zero());
                                                    }
                                                }
                                                effect_invalidator.send(EffectInvalidate);
                                            },
                                        },
                                        ":",
                                        input {
                                            class: "seconds",
                                            r#type: "number",
                                            value: effect.duration.num_seconds() % 60,
                                            onchange: move |v| {
                                                let seconds = effect.duration.num_seconds() % 60;
                                                let new_seconds = v.value().parse::<i64>().unwrap_or(0);
                                                let delta = new_seconds - seconds;
                                                log::info!("Delta: {delta}");
                                                {
                                                    let mut w = current_effect.write();
                                                    if let Some(w) = &mut *w {
                                                        w.duration += Duration::seconds(delta);
                                                        w.duration = w.duration.max(Duration::zero());
                                                    }
                                                }
                                                effect_invalidator.send(EffectInvalidate);
                                            },
                                        },
                                        ".",
                                        input {
                                            class: "milliseconds",
                                            r#type: "number",
                                            value: effect.duration.num_milliseconds() % 1000,
                                            onchange: move |v| {
                                                let milliseconds = effect.duration.num_milliseconds() % 1000;
                                                let new_milliseconds = v.value().parse::<i64>().unwrap_or(0);
                                                let delta = new_milliseconds - milliseconds;
                                                log::info!("Delta: {delta}");
                                                {
                                                    let mut w = current_effect.write();
                                                    if let Some(w) = &mut *w {
                                                        w.duration += Duration::milliseconds(delta);
                                                        w.duration = w.duration.max(Duration::zero());
                                                    }
                                                }
                                                effect_invalidator.send(EffectInvalidate);
                                            },
                                        },
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

#[component]
fn to_visualized_effect_name(name: String) -> Element {
    let paths = use_memo(use_reactive!(|name| {
        let paths = name.split('/').map(|p| p.to_string()).collect::<Vec<_>>();
        let r: &[String] = &paths;
        match r {
            [] => unreachable!(),
            [rest @ .., name] => (rest.to_vec(), name.clone()),
        }
    }));
    rsx! {
        p { class: "visualized-effect-name",
            for r in paths().0 {
                span { class: "folder", {r} }
                span { class: "divider", "/" }
            }
            span { class: "name", {paths().1} }
        }
    }
}
