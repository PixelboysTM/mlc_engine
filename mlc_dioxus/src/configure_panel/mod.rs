use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::Message;

use fixture_tester::FixtureTester;
use mlc_common::endpoints::{EPConfigItem, EndPointConfig, Speed};
use mlc_common::patched::{PatchedFixture, UniverseAddress, UniverseId};
use mlc_common::universe::FixtureUniverse;
use mlc_common::{
    FaderUpdateRequest, FixtureInfo, Info, PatchResult, ProjectDefinition, ProjectSettings,
    RuntimeUpdate,
};

use crate::utils::toaster::{Toaster, ToasterWriter};
use crate::utils::{CheckboxState, Loading};
use crate::{icons, utils};

mod fixture_tester;

#[component]
pub fn ConfigurePanel() -> Element {
    let project_info = use_resource(|| utils::fetch::<ProjectDefinition>("/projects/current"));

    rsx! {
        div { class: "configure-panel",
            div { class: "panel info",

                h3 { class: "header", "Project Info" }
                match &*project_info.read_unchecked() {
                    Some(Ok(d)) => {rsx!{
                        div {
                            class: "bin-ico",
                            title: if d.binary {"Compressed"} else {"Uncompressed"},
                            {if d.binary {
                                rsx! {
                                    icons::FileArchive {}
                                }
                            } else {
                                rsx!{
                                    icons::FileJson {}
                                }
                            }}
                        }
                        p {
                            span {
                                class: "pis",
                                "Name: "
                            },
                            {d.name.clone()}
                        },
                        p {
                            span {
                                class: "pis",
                                "Filename: "
                            },
                            {d.file_name.clone()}
                        },
                        p {
                            span {
                                class: "pis",
                                "Last Saved: "
                            },
                            {d.last_edited.format("%d.%m.%Y %H:%M:%S").to_string()}
                        }
                    }},
                    Some(Err(_e)) => {rsx!{"Error Loading Project Info"}},
                    None => {Loading()}
                }
            }
            div { class: "panel fixture-types", FixtureTypeExplorer {} }
            div { class: "panel universe-explorer", UniverseExplorer {} }
            div { class: "panel project-settings", ProjectSettings {} }
            div { class: "panel fader-browser", FaderPanel {} }
        }
    }
}

#[component]
fn ProjectSettings() -> Element {
    let settings = use_resource(|| utils::fetch::<ProjectSettings>("/settings/get"));
    let mut endpoint_mapping = use_signal(|| false);
    let mut changed_settings = use_signal(|| None);
    rsx! {
        if endpoint_mapping() {
            EndPointMapping {
                onclose: move |_| {
                    endpoint_mapping.set(false);
                }
            }
        }
        div { class: "project-settings-panel",
            h3 { class: "header", "Project Settings" }
            div { class: "settings",
                if changed_settings().is_some() {
                    div { class: "unsaved",
                        p { "Unsaved changes press Update to Confirm." }
                    }
                }

                match &*settings.read_unchecked() {
                    Some(Ok(s)) => {
                            rsx!{
                                div {
                                    class: "setting",
                                    p {
                                        "Save on quit"
                                    },
                                    input {
                                        r#type: "checkbox",
                                        checked: s.save_on_quit,
                                        onchange: move |v| {
                                            log::info!("{}", v.data.value());
                                            if let Some(_ss) = changed_settings() {
                                                changed_settings.set(Some(ProjectSettings{
                                                save_on_quit: v.data.value() == "true",
                                                }))
                                            } else {
                                                changed_settings.set(Some(ProjectSettings{
                                                    save_on_quit: v.data.value() == "true",
                                                }))
                                            }
                                        }
                                    }
                                }
                            }
                
                    }
                    Some(Err(_s)) => {rsx!("Error Fetching settings")}
                    None => {utils::Loading()}
                }
            }
            div { class: "btns",
                button {
                    title: "Endpoints",
                    onclick: move |_| {
                        endpoint_mapping.set(true);
                    },
                    icons::Cable { width: "1rem", height: "1rem" }
                }
                button {
                    onclick: move |_| {
                        async move {
                            if let Some(s) = changed_settings() {
                                let s = utils::fetch_post::<String, _>("/settings/update", s).await;
                                if s.is_ok() {
                                    changed_settings.set(None);
                                } else {
                                    log::error!("{:?}", s);
                                }
                            }
                        }
                    },
                    "Update"
                }
            }
        }
    }
}

#[component]
fn FaderPanel() -> Element {
    let mut current_universe = use_signal(|| 1);
    let mut universes = use_resource(|| async move {
        if let Ok(d) = utils::fetch::<Vec<u16>>("/data/universes").await {
            d
        } else {
            vec![]
        }
    });
    let info = use_context::<Signal<Info>>();
    use_effect(move || {
        if info() == Info::UniversesUpdated {
            universes.restart();
        }
    });

    let mut current_values = use_signal(|| [0_u8; 512]);

    let mut started = use_signal(|| false);
    let get = use_coroutine(|mut rx: UnboundedReceiver<u16>| async move {
        if started() {
            return;
        }
        started.set(true);

        let ws_o = utils::ws("/runtime/fader-values/get").await;

        if let Ok(get_ws) = ws_o {
            let mut get_ws = get_ws.fuse();
            loop {
                futures::select! {
                    msg = rx.next() => {
                        if let Some(msg) = msg {
                            let _ = get_ws.send(Message::Text(msg.to_string())).await;
                        }
                    },
                    msg = get_ws.next() => {
                        let d = match msg {
                            Some(Ok(msg)) => {
                                match msg {
                                    Message::Text(t) => serde_json::from_str::<RuntimeUpdate>(&t).ok(),
                                    Message::Bytes(b) => serde_json::from_str::<RuntimeUpdate>(
                                        &String::from_utf8(b).unwrap(),
                                    ).ok(),
                                }
                            }
                            Some(Err(e)) => {
                                let e: gloo_net::websocket::WebSocketError = e;
                                match e {
                                    gloo_net::websocket::WebSocketError::ConnectionClose(c) => {
                                        log::info!("WS was closed code: {}", c.code);
                                    }
                                    e => {
                                        log::error!("Websocket error: {e:?}");
                                    }
                                }
                                None
                            }
                            None => {
                                None
                            }
                        };

                        if let Some(update) = d {
                            match update {
                                RuntimeUpdate::ValueUpdated {
                                    universe,
                                    channel_index,
                                    value,
                                } => {
                                    if current_universe.read().deref() == &universe.0 {
                                        current_values.with_mut(|g| g[channel_index] = value);
                                    }
                                }
                                RuntimeUpdate::ValuesUpdated {
                                    universes,
                                    channel_indexes,
                                    values,
                                } => {
                                    current_values.with_mut(|g| {
                                        for (i, index) in channel_indexes.iter().enumerate() {
                                            if current_universe.read().deref() == &universes[i].0 {
                                                g[*index] = values[i];
                                            }
                                        }
                                    });
                                }
                                RuntimeUpdate::Universe {
                                    universe, values, ..
                                } => {
                                    if current_universe() == universe.0 {
                                        current_values.set(values);
                                    }
                                }
                            };
                        };
                    }
                }
            }
        } else {
            log::error!("Error creating {:?}", ws_o.err().unwrap());
        }
    });
    let set = use_coroutine(|mut rx: UnboundedReceiver<FaderUpdateRequest>| async move {
        let ws = utils::ws("/runtime/fader-values/set").await;

        if let Ok(mut ws) = ws {
            loop {
                let m = rx.next().await;
                if let Some(r) = m {
                    let _ = ws
                        .send(Message::Text(serde_json::to_string(&r).unwrap()))
                        .await;
                }
            }
        } else {
            log::error!("Error opening websocket: {:?}", ws.err().unwrap());
        }
    });
    rsx! {
        div { class: "slider-panel",
            div { class: "universe-list",
                match universes.read_unchecked().as_ref().cloned() {
                    Some(us) => {
                        rsx!{
                            for u in us {
                                div {
                                    class: "tab {sel(current_universe() == u)}",
                                    onclick: move |_| {
                                        current_universe.set(u);
                                        get.send(u);
                                    },
                                    {u.to_string()}
                                }
                            }
                        }
                    }
                    None => {rsx!(Loading{})}
                }
            }
            div { class: "faders",
                {(0..512).map(|i| {
                    rsx!{
                        Fader{
                        value: current_values.read()[i],
                        id: make_three_digit(i as u16),
                        onchange: move |v| {
                                set.send(FaderUpdateRequest{
                                    universe: UniverseId(*current_universe.read().deref()),
                                    channel: UniverseAddress::create(i as u16).expect("Must be"),
                                    value: v
                                });
                        },
                    }
                    }
                })}
            }
        }
    }
}

#[component]
pub fn Fader(value: u8, id: String, onchange: EventHandler<u8>) -> Element {
    let mut val = use_signal(|| value);
    use_effect(use_reactive((&value,), move |(v,)| val.set(v)));

    let mut size_e = use_signal(|| None);
    rsx! {
        div { class: "fader-container",
            div { class: "name", {id} }

            div {
                class: "range",
                background: "linear-gradient(0deg, var(--color-gradient-start) 0%, var(--color-gradient-end) {(val() as f32 / 255.0) * 100.0}%, transparent {(val() as f32 / 255.0) * 100.0}%, transparent 100%)",
                onmounted: move |e| {
                    size_e.set(Some(e.data));
                },
                onmousemove: move |e| {
                    async move {
                        if e.held_buttons() == MouseButton::Primary {
                            let size = size_e()
                                .unwrap()
                                .get_client_rect()
                                .await
                                .unwrap()
                                .size
                                .height;
                            let p = e.data.element_coordinates();
                            let x = (1.0 - p.y / size).min(1.0).max(0.0);
                            let v = (x * 255.0) as u8;
                            val.set(v);
                            onchange.call(v);
                        }
                    }
                },
                onmousedown: move |e| {
                    async move {
                        if e.data.held_buttons() == MouseButton::Primary {
                            let size = size_e()
                                .unwrap()
                                .get_client_rect()
                                .await
                                .unwrap()
                                .size
                                .height;
                            let p = e.data.element_coordinates();
                            let x = (1.0 - p.y / size).min(1.0).max(0.0);
                            let v = (x * 255.0) as u8;
                            val.set(v);
                            onchange.call(v);
                        }
                    }
                }
            }

            div { class: "value", {make_three_digit(val() as u16)} }
        }
    }
}

fn make_three_digit(u: u16) -> String {
    format!("{:03}", u)
}

fn sel(b: bool) -> &'static str {
    if b {
        "sel"
    } else {
        ""
    }
}

#[component]
fn FixtureTypeExplorer() -> Element {
    let mut fixture_query = use_resource(|| async move {
        let r = utils::fetch::<Vec<FixtureInfo>>("/data/get/fixture-types").await;
        if let Ok(infos) = r {
            infos
        } else {
            log::error!("Couldn't fetch types: {:?}", r.err().unwrap());
            vec![]
        }
    });

    let info = use_context::<Signal<Info>>();
    use_effect(move || {
        if info() == Info::FixtureTypesUpdated {
            fixture_query.restart();
        }
    });

    let mut detail_fixture = use_signal::<Option<FixtureInfo>>(|| None);

    rsx! {
        if let Some(f) = detail_fixture() {
            DetailFixtureType {
                t: f,
                onclose: move |_| {
                    detail_fixture.set(None);
                }
            }
        }

        div { class: "fixture-type-explorer",
            h3 { class: "header", "Fixture Types" }

            match fixture_query.read_unchecked().as_ref().cloned() {
                Some(infos) => {
                        rsx!{
                    for info in infos {
                            div {
                                class: "fixture-type",
                                onclick: move |_| {
                                    detail_fixture.set(Some(info.clone()));
                                },
                                h3 {
                                    class: "name",
                                    {info.name.clone()}
                                },
                                p {
                                    class: "id",
                                    {info.id.to_string()}
                                },
                                div {
                                    class: "modes",
            
                                    for mode in &info.modes {
                                        li {
                                            class: "mode",
                                            {mode.name.clone()}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None => {utils::Loading()}
            }
        }
    }
}

#[component]
fn DetailFixtureType(t: FixtureInfo, onclose: EventHandler) -> Element {
    let inf = use_memo(move || t.clone());

    let mut create_new_universe = use_signal(|| true);
    let mut sel_mode = use_signal(|| {
        inf()
            .modes
            .first()
            .map(|m| m.short_name.clone())
            .unwrap_or("No modes".to_string())
    });

    let sel_mode_o = use_memo(move || {
        inf()
            .modes
            .iter()
            .find(|mo| mo.short_name == sel_mode())
            .cloned()
    });

    rsx! {
        utils::Overlay {
            title: inf().name.clone(),
            class: "fixture-type-detail",
            icon: rsx! {
                icons::Blocks {}
            },
            onclose: move |_| {
                onclose.call(());
            },
            div { class: "settings",
                span { "Create Additional Universes" }
                utils::Checkbox {
                    init: create_new_universe().into(),
                    onchange: move |s: CheckboxState| {
                        create_new_universe.set(s.into());
                    }
                }
            }
            div { class: "modes",
                for mode in inf().modes {
                    div {
                        class: "mode {sel(mode.short_name == sel_mode())}",
                        onclick: move |e| {
                            if e.trigger_button() == Some(MouseButton::Primary)
                                && mode.short_name != sel_mode()
                            {
                                sel_mode.set(mode.short_name.clone());
                            }
                        },
                        {mode.short_name.clone()}
                    }
                }
            }
            div { class: "detail",
                {
                    if let Some(mode) = sel_mode_o() {
                        rsx!{
                            h3 {
                                class: "mode-name",
                                {format!("{} ({})", mode.name, mode.short_name)}
                            },
                            for (i, channel) in mode.channels.iter().enumerate() {
                                p {
                                    class: "channel",
                                    {format!("{}: {}", i, channel)}
                                }
                            },
                            div {
                                class: "buttons",
                                button {
                                    onclick: move |_| {
                                        let mode = mode.clone();
                                        async move {
                                            let mut toaster = use_context::<Signal<Toaster>>();
                                            let id = inf().id;
                                            let i = inf().modes
                                            .iter()
                                            .enumerate()
                                            .find(|(_, m)| m.short_name == mode.short_name)
                                            .map(|(i, _)| i)
                                            .unwrap_or(0);
                                            onclose.call(());
                                            let e = utils::fetch::<PatchResult>(&format!("/data/patch/{}/{}?create={}",id,i,create_new_universe())).await;
                                            match e {
                                                Ok(PatchResult::Failed(msg)) => {
                                                    let _ = toaster.error("Failed patching", format!("Failed to patch fixtures.\n{msg}"));
                                                },
                                                Ok(PatchResult::Success(msg)) => {
                                                    let _ = toaster.info("Patched", format!("Successful.\n{msg}"));
                                                },
                                                Ok(PatchResult::ModeInvalid(msg)) => {
                                                    let _ = toaster.error("Failed patching", format!("The specified mode is not valid.\n{msg}"));
                                                },
                                                Ok(PatchResult::IdInvalid(msg)) => {
                                                    let _ = toaster.error("Failed patching", format!("The specified FixtureId is not valid.\n{msg}"));
                                                },
                                                Err(e) => {let _ = toaster.error("WTF", format!("I don't even know what happened good luck!\n{e:?}"));}
                                            }
                                        }
                                    },
                                    "Patch"
                                },
                                button {
                                    onclick: move |_| {
                                        onclose.call(());
                                    },
                                    "Close"
                                }
                            }
                        }
                    } else {
                        rsx!("")
                    }
                }
            }
            div { class: "fix-id", {inf().id.to_string()} }
        }
    }
}

#[component]
fn UniverseExplorer() -> Element {
    let mut universes = use_resource(|| async move {
        utils::fetch::<Vec<UniverseId>>("/data/universes")
            .await
            .map_err(|e| {
                log::error!("Error fetching universes: {:?}", e);
            })
            .unwrap_or(vec![])
    });

    let mut selected = use_signal(|| UniverseId(1));
    let universe = use_resource(move || async move {
        utils::fetch::<FixtureUniverse>(&format!("/data/universes/{}", selected().0))
            .await
            .map_err(|e| {
                log::error!("Error fetching universes: {:?}", e);
            })
            .ok()
    });

    let mut detail_fixture = use_signal::<Option<PatchedFixture>>(|| None);
    let mut detail_fixture_id = use_signal(|| None);
    use_effect(move || {
        if let Some(id) = detail_fixture_id() {
            let u = universe.value()().expect("Must be").expect("Must be");
            let val = u.fixtures.get(id).cloned();
            detail_fixture.set(val);
            detail_fixture_id.set(None);
        }
    });

    let info = use_context::<Signal<Info>>();
    use_effect(move || match info() {
        Info::UniversePatchChanged(id) => {
            if id == *selected.peek() {
                selected.set(id);
            }
        }
        Info::UniversesUpdated => {
            universes.restart();
        }
        _ => {}
    });

    match universes.read_unchecked().as_ref().cloned() {
        Some(d) => {
            rsx! {
                if let Some(f) = detail_fixture() {
                    FixtureTester {
                        info: f.clone(),
                        onclose: move |_| {
                            detail_fixture.set(None);
                        }
                    }
                }

                h3 { class: "header", "Universe Explorer" }
                div { class: "universe-explorer-container",
                    div { class: "tabs",
                        for id in d {
                            div {
                                class: "tab {sel(selected() == id)}",
                                onclick: move |_| {
                                    selected.set(id);
                                },
                                {id.0.to_string()}
                            }
                        }
                    }
                    match universe.value()() {
                        Some(Some(data)) => {
                            rsx!{
                                div {
                                    class: "channels",
                                    for (i, channel) in data.channels.iter().cloned().enumerate() {
                                        match channel {
                                            Some(c) => {
                                                rsx! {
                                                    div {
                                                        class: "patched-channel {channel_type(data.fixtures[c.fixture_index].num_channels as usize, c.channel_index)}",
                                                        title: data.fixtures[c.fixture_index].name.clone(),
                                                        onclick: move |e| {
                                                            if let Some(MouseButton::Primary) = e.trigger_button(){
                                                                detail_fixture_id.set(Some(c.fixture_index))
                                                            }
                                                            if let Some(MouseButton::Secondary) = e.trigger_button(){
                                                                log::info!("Right Click");
                                                            }
                                                        },
                                                        if c.channel_index == 0 {
                                                            code {
                                                                {i.to_string()}
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            None => {
                                                rsx! {
                                                    div {
                                                        class: "channel",
                                                        code {
                                                            {i.to_string()}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    },
                                }
                            }
                        }
                        Some(None) => {
                            rsx!{
                                p {
                                    "Error loading universe data"
                                }
                            }
                        }
                        None => utils::Loading()
                    }
                }
            }
        }
        None => utils::Loading(),
    }
}

fn channel_type(amount: usize, i: usize) -> &'static str {
    if i == 0 {
        if amount == 1 {
            "start end"
        } else {
            "start"
        }
    } else if i == amount - 1 {
        "end"
    } else {
        "middle"
    }
}

#[derive(PartialEq, Copy, Clone)]
enum FixtureSource {
    Ofl,
    Json,
}

#[derive(serde::Serialize)]
#[allow(non_snake_case)]
struct SearchBody {
    searchQuery: &'static str,
    manufacturersQuery: [&'static str; 0],
    categoriesQuery: [&'static str; 0],
}

#[derive(serde::Deserialize, Clone)]
struct AvailableFixture {
    manufacturer: String,
    name: String,
}

#[component]
pub fn UploadFixturePopup(on_close: EventHandler<()>) -> Element {
    let mut source = use_signal(|| FixtureSource::Ofl);

    let available_fixtures = use_resource(move || async move {
        let r = utils::fetch_post::<Vec<String>, _>(
            "https://open-fixture-library.org/api/v1/get-search-results",
            SearchBody {
                searchQuery: "",
                categoriesQuery: [],
                manufacturersQuery: [],
            },
        )
        .await;

        let result = r
            .map_err(|e| log::error!("Error fetching available fixtures: {:?}", e))
            .ok()
            .map(|v| {
                v.iter()
                    .map(|e| {
                        let mut s = e.split('/');
                        AvailableFixture {
                            manufacturer: s.next().unwrap().to_string(),
                            name: s.next().unwrap().to_string(),
                        }
                    })
                    .collect::<Vec<_>>()
            });

        result.expect("Must be")
    });

    let mut search = use_signal(|| "".to_string());

    rsx! {
        utils::Overlay {
            title: "Import Fixture Types",
            class: "upload-fixture",
            icon: rsx! {
                icons::LampDesk {}
            },
            onclose: move |_| {
                on_close.call(());
            },

            div { class: "tabs",

                div {
                    class: "tab {sel(source() == FixtureSource::Ofl)}",
                    onclick: move |_| {
                        source.set(FixtureSource::Ofl);
                    },
                    "OFL"
                }
                div {
                    class: "tab {sel(source() == FixtureSource::Json)}",
                    onclick: move |_| {
                        source.set(FixtureSource::Json);
                    },
                    "RAW"
                }
            }

            div { class: "list-content",
                match source() {
                    FixtureSource::Ofl => {
                        rsx! {
                        match available_fixtures.state()() {
                
                        UseResourceState::Pending => {
                                rsx!(utils::Loading {})
                            }
                            UseResourceState::Stopped | UseResourceState::Paused => {
                                rsx!{"Failed to query fixtures from ofl. Is your device connected to the internet?"}}
                            UseResourceState::Ready => {
                                rsx! {
                                    div {
                                        class: "searchbar",
                                        input {
                                            r#type: "text",
                                            onchange: move |e| {
                                                search.set(e.data.value());
                                            },
                                            oninput: move |e| {
                                                search.set(e.data.value());
                                            }
                                        }
                                    },
                                    div {
                                        class: "results",
                                        for available in filter_search(available_fixtures.read().clone().unwrap_or(vec![]), &search()) {
                                            div {
                                                class: "result",
                                                p {
                                                    class: "manufacturer",
                                                    {available.manufacturer.clone()}
                                                },
                                                p {
                                                    class: "name",
                                                    {available.name.clone()}
                                                },
                
                                                button {
                                                    class: "icon",
                                                    title: "Import",
                                                    onclick: move |_| {
                                                        let m = available.manufacturer.clone();
                                                        let n = available.name.clone();
                                                        async move {
                                                            log::info!("Import fixture");
                                                            let _ = utils::fetch::<()>(&format!("/data/add/fixture-ofl/{}/{}", m, n)).await.map_err(|e| {
                                                                log::error!("Error importing: {:?}", e);
                                                            });
                                                        }
                                                    },
                                                    icons::Download {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    }
                    FixtureSource::Json => {rsx!{
                        "Currently not available"
                    }}
                }
            }
        }
    }
}

fn fits_search(f: &AvailableFixture, search: &str) -> bool {
    let i = format!(
        "{}/{}",
        f.manufacturer.to_lowercase(),
        f.name.to_lowercase()
    );
    let keywords = search.split(' ');
    for keyword in keywords {
        if !i.contains(&keyword.to_lowercase()) {
            return false;
        }
    }

    true
}

fn filter_search(fs: Vec<AvailableFixture>, search: &str) -> Vec<AvailableFixture> {
    let mut r = vec![];
    for f in fs {
        if fits_search(&f, search) {
            r.push(f.clone());
        }
    }

    r
}

#[component]
fn EndPointMapping(onclose: EventHandler) -> Element {
    let mut config = use_resource(|| async move {
        let r = utils::fetch::<EndPointConfig>("/runtime/endpoints/get").await;
        match r {
            Ok(c) => {
                let us = utils::fetch::<Vec<UniverseId>>("/data/universes").await;
                us.map(|us| (us, c)).ok()
            }
            Err(e) => {
                log::error!("Error fetching endpoint config: {:?}", e);
                None
            }
        }
    });

    let mut transformed_config = use_signal(|| None);
    use_effect(move || {
        let r = config().map(|c| {
            c.map(|(us, ep_config)| {
                us.iter()
                    .map(|u| (*u, ep_config.endpoints.get(u).cloned().unwrap_or(vec![])))
                    .collect::<Vec<_>>()
            })
        });
        transformed_config.set(r);
    });

    let mut toaster = use_context::<Signal<Toaster>>();

    let info = use_context::<Signal<Info>>();
    use_effect(move || {
        if info() == Info::EndpointConfigChanged || info() == Info::UniversesUpdated {
            config.restart();
        }
    });

    rsx! {
        utils::Overlay {
            title: "Endpoint Mapping",
            class: "endpoint-mapping",
            icon: rsx! {
                icons::Cable {}
            },
            onclose: move |_| {
                onclose.call(());
            },

            match transformed_config() {
                Some(Some(us)) => {
                    rsx!{
                        for (u, eps) in us {
                            div {
                                class: "universe",
                                p {
                                    class: "universe-id",
                                    {format!("Universe: {}", u.0)}
                                },
                                div {
                                    class: "endpoints",
                                    for (i, ep) in eps.into_iter().enumerate() {
                                        div {
                                            class: "endpoint",
                                            div {
                                                class: "endpoint-type",
                                                div {
                                                    class: "sacn",
                                                    class: if matches!(&ep, EPConfigItem::Sacn {..}) {"sel"},
                                                    title: "sACN",
                                                    onclick: make_type_closure(ep.clone(), move |_, ep| {
                                                        if !matches!(ep, EPConfigItem::Sacn{..}) {
                                                            let mut w = transformed_config.write();
                                                            let c = w.as_mut().expect("").as_mut().expect("");
                                                            for (uid, conf) in c {
                                                                if *uid == u {
                                                                    conf[i] = EPConfigItem::Sacn {
                                                                        universe: 1,
                                                                        speed: Speed::Medium,
                                                                    };
                                                                }
                                                            }
                                                        }
                                                    }),
                                                    icons::Wand{
                                                        width: "1rem",
                                                        height: "1rem"
                                                    },
                                                },
                                                div {
                                                    class: "log",
                                                    class: if matches!(&ep, EPConfigItem::Logger) {"sel"},
                                                    title: "Logger",
                                                    onclick: make_type_closure(ep.clone(), move  |_, ep| {
                                                        if !matches!(ep, EPConfigItem::Logger) {
                                                            let mut w = transformed_config.write();
                                                            let c = w.as_mut().expect("").as_mut().expect("");
                                                            for (uid, conf) in c {
                                                                if *uid == u {
                                                                    conf[i] = EPConfigItem::Logger;
                                                                }
                                                            }
                                                        }
                                                    }),
                                                    icons::MessageCircleQuestion{
                                                        width: "1rem",
                                                        height: "1rem"
                                                    },
                                                },
                                                div {
                                                    class: "artnet",
                                                    class: if matches!(&ep, EPConfigItem::ArtNet) {"sel"},
                                                    title: "ArtNet",
                                                    onclick: make_type_closure(ep.clone(), move |_, ep| {
                                                        if !matches!(ep, EPConfigItem::ArtNet) {
                                                            let mut w = transformed_config.write();
                                                            let c = w.as_mut().expect("").as_mut().expect("");
                                                            for (uid, conf) in c {
                                                                if *uid == u {
                                                                    conf[i] = EPConfigItem::ArtNet;
                                                                }
                                                            }
                                                        }
                                                    }),
                                                    icons::Palette{
                                                        width: "1rem",
                                                        height: "1rem"
                                                    },
                                                },
                                                div {
                                                    class: "usb",
                                                    class: if matches!(&ep, EPConfigItem::Usb{..}) {"sel"},
                                                    title: "Usb",
                                                    onclick: make_type_closure(ep.clone(), move |_, ep| {
                                                        if !matches!(ep, EPConfigItem::Usb{..}) {
                                                            let mut w = transformed_config.write();
                                                            let c = w.as_mut().expect("").as_mut().expect("");
                                                            for (uid, conf) in c {
                                                                if *uid == u {
                                                                    conf[i] = EPConfigItem::Usb {
                                                                        port: "COM1".to_string(),
                                                                        speed: Speed::Medium,
                                                                    };
                                                                }
                                                            }
                                                        }
                                                    }),
                                                    icons::Usb{
                                                        width: "1rem",
                                                        height: "1rem"
                                                    },
                                                }
                                            },
                                            div {
                                                class: "content",
                                                {match ep {
                                                    EPConfigItem::Logger => {
                                                        rsx! {
                                                            p {
                                                                "Logger (no config needed)",
                                                            }
                                                        }
                                                    }
                                                    EPConfigItem::ArtNet => {
                                                        rsx! {
                                                            p {
                                                                "ArtNet (no config needed)",
                                                            }
                                                        }
                                                    }
                                                    EPConfigItem::Sacn{ universe, speed } => {
                                                        rsx! {
                                                            p {
                                                                "sACN",
                                                            },
                                                            div {
                                                                class: "property",
                                                                p {
                                                                    "Universe:",
                                                                },
                                                                input {
                                                                    r#type: "number",
                                                                    value: universe as i64,
                                                                    min: 1,
                                                                    oninput: move |e| {
                                                                        let mut w = transformed_config.write();
                                                                        let c = w.as_mut().expect("").as_mut().expect("");
                                                                        for (uid, conf) in c {
                                                                            if *uid == u {
                                                                                let item = conf.get_mut(i).expect("");
                                                                                if let EPConfigItem::Sacn{universe, ..} = item {
                                                                                    *universe = u16::from_str(&e.value()).unwrap_or(1).max(1);
                                                                                    needs_update();
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                }
                                                            },
                                                            div {
                                                                class: "property",
                                                                p {
                                                                    "Speed:",
                                                                },
                                                                select {
                                                                    value: format!("\"{:?}\"", speed),
                                                                    onchange: move |e| {
                                                                        let mut w = transformed_config.write();
                                                                        let c = w.as_mut().expect("").as_mut().expect("");
                                                                        for (uid, conf) in c {
                                                                            if *uid == u {
                                                                                let item = conf.get_mut(i).expect("");
                                                                                if let EPConfigItem::Sacn{universe: _, speed} = item {
                                                                                    *speed = serde_json::from_str(&e.value()).unwrap_or(Speed::Medium);
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    option {
                                                                        value: "\"Fast\"",
                                                                        "Fast"
                                                                    },
                                                                    option {
                                                                        value: "\"Medium\"",
                                                                        "Medium"
                                                                    },
                                                                    option {
                                                                        value: "\"Slow\"",
                                                                        "Slow"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    EPConfigItem::Usb{ ref port, speed } => {
                                                        rsx! {
                                                            p {
                                                                "sACN",
                                                            },
                                                            div {
                                                                class: "property",
                                                                p {
                                                                    "Port:",
                                                                },
                                                                input {
                                                                    r#type: "text",
                                                                    value: port.clone(),
                                                                    oninput: move |e| {
                                                                        let mut w = transformed_config.write();
                                                                        let c = w.as_mut().expect("").as_mut().expect("");
                                                                        for (uid, conf) in c {
                                                                            if *uid == u {
                                                                                let item = conf.get_mut(i).expect("");
                                                                                if let EPConfigItem::Usb{ port, ..} = item {
                                                                                    *port = e.value();
                                                                                    needs_update();
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                }
                                                            },
                                                            div {
                                                                class: "property",
                                                                p {
                                                                    "Speed:",
                                                                },
                                                                select {
                                                                    value: format!("\"{:?}\"", speed),
                                                                    onchange: move |e| {
                                                                        let mut w = transformed_config.write();
                                                                        let c = w.as_mut().expect("").as_mut().expect("");
                                                                        for (uid, conf) in c {
                                                                            if *uid == u {
                                                                                let item = conf.get_mut(i).expect("");
                                                                                if let EPConfigItem::Usb{ speed, ..} = item {
                                                                                    *speed = serde_json::from_str(&e.value()).unwrap_or(Speed::Medium);
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    option {
                                                                        value: "\"Fast\"",
                                                                        "Fast"
                                                                    },
                                                                    option {
                                                                        value: "\"Medium\"",
                                                                        "Medium"
                                                                    },
                                                                    option {
                                                                        value: "\"Slow\"",
                                                                        "Slow"
                                                                    },
                                                                    option {
                                                                        value: "\"SuperFast\"",
                                                                        "Super Fast"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }}
                                            },
                                            button {
                                                class: "icon delete-btn",
                                                onclick: move |_| {
                                                    let mut w = transformed_config.write();
                                                    let c = w.as_mut().expect("").as_mut().expect("");
                                                    for (uid, conf) in c {
                                                        if *uid == u {
                                                            conf.remove(i);
                                                        }
                                                    }
                                                },
                                                icons::Trash2 {
                                                    width: "1rem",
                                                    height: "1rem",
                                                }
                                            }
                                        }
                                    }
                                },
                                button {
                                    class: "add-endpoint-btn icon",
                                    title: "Add new Endpoint",
                                    onclick: move |_| {
                                        let mut w = transformed_config.write();
                                        let c = w.as_mut().expect("").as_mut().expect("");
                                        for (uid, conf) in c {
                                            if *uid == u {
                                                conf.push(EPConfigItem::Logger);
                                            }
                                        }
                                    },
                                    icons::Plus {}
                                }
                            }
                        }
            
                        div {
                            class: "btns",
                            button {
                                onclick: move |_| {
                                    async move {
                                        let w = transformed_config();
                                        if let Some(Some(c)) = w {
                                            let mut map = HashMap::new();
                                            for (id, conf) in c {
                                                if !conf.is_empty() {
                                                    map.insert(id, conf);
                                                }
                                            }
                                            let ep_config = EndPointConfig {
                                                endpoints: map,
                                            };
                                            let r = utils::fetch_post::<String, _>("/runtime/endpoints/set", ep_config).await;
                                            if r.is_ok() {
                                                onclose.call(());
                                            } else {
                                                toaster.error("Endpoint config Update failed", "Failed to apply new endpoint config. See Backend output for more Information");
                                            }
                                        }
                                    }
                                },
                                "Apply"
                            },
                            button {
                                onclick: move |_| {
                                    onclose.call(())
                                },
                                "Cancel"
                            }
                        }
                    }
                }
                Some(None) => {rsx!{"Error fetching config see console for more information!"}}
                None => {
                    rsx!{utils::Loading{}}
                }
            }
        }
    }
}

fn make_type_closure<F, E, T>(
    ep: EPConfigItem,
    mut closure: F,
) -> impl FnMut(Event<MouseData>) -> E + 'static
where
    E: EventReturn<T>,
    F: FnMut(Event<MouseData>, EPConfigItem) -> E + 'static,
{
    move |data| closure(data, ep.clone())
}
