use std::{thread, time::Duration};

use crossbeam::channel::{Receiver, Sender};
use rocket::{
    data::ToByteUnit,
    futures::SinkExt,
    get, post,
    response::{content::RawJson, status::BadRequest},
    routes, Data, Route, State,
};
use rocket_ws::WebSocket;
use uuid::Uuid;

use crate::{
    data_spreader::{DataSender, DataSubscriber},
    fixture,
    project::Project,
};

pub struct InfoRx {
    pub rx: Receiver<Info>,
}

pub struct InfoTx {
    pub tx: Sender<Info>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub enum Info {
    FixtureTypesUpdated,
    ProjectSaved,
    ProjectLoaded,
}

#[get("/info")]
async fn gen_info(ws: WebSocket, rx: &State<DataSubscriber<Info, Uuid>>) -> rocket_ws::Channel<'_> {
    let rx = rx.subscribe().await;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                while let Some(msg) = rx.recv().await {
                    println!("{:?}", msg);
                    let _ = stream
                        .send(rocket_ws::Message::Text(
                            serde_json::to_string(&msg).unwrap(),
                        ))
                        .await;
                }
            }
        })
    })
}

#[get("/get/fixture-types")]
async fn get_fixture_types(project: &State<Project>) -> RawJson<String> {
    thread::sleep(Duration::new(1, 0));

    RawJson(
        serde_json::to_string(
            &project
                .inner()
                .get_fixtures()
                .await
                .iter()
                .map(|f| f.get_name())
                .collect::<Vec<&str>>(),
        )
        .unwrap(),
    )
}

#[post("/add/fixture", data = "<data>")]
async fn add_fixture(
    data: Data<'_>,
    project: &State<Project>,
    info: &State<DataSender<Info>>,
) -> Result<(), BadRequest<String>> {
    let s = data.open(2.gibibytes());
    let string = s
        .into_string()
        .await
        .map_err(|_| BadRequest("Failed to read to string".to_string()))?;

    let fix = fixture::parse_ofl_fixture(&string).map_err(|e| BadRequest(e))?;

    project.insert_fixture(fix, &info).await;

    Ok(())
}

pub fn create_info() -> (InfoRx, InfoTx) {
    let (tx, rx) = crossbeam::channel::unbounded();

    (InfoRx { rx }, InfoTx { tx })
}

pub fn get_routes() -> Vec<Route> {
    routes![gen_info, get_fixture_types, add_fixture]
}
