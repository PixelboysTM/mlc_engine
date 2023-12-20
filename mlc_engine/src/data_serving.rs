use std::{thread, time::Duration};

use rocket::{
    data::ToByteUnit,
    futures::SinkExt,
    get, post,
    response::{content::RawJson, status::BadRequest},
    routes,
    tokio::{select, sync::broadcast::Sender},
    Data, Route, State,
};
use rocket_ws::WebSocket;

use crate::{fixture, module::Module, project::Project};

#[derive(serde::Serialize, Debug, Clone)]
pub enum Info {
    FixtureTypesUpdated,
    ProjectSaved,
    ProjectLoaded,
    SystemShutdown,
}

#[get("/info")]
async fn gen_info(
    ws: WebSocket,
    tx: &State<Sender<Info>>,
    mut shutdown: rocket::Shutdown,
) -> rocket_ws::Channel<'_> {
    let mut rx = tx.subscribe();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    Ok(msg) = rx.recv() => {
                        println!("{:?}", msg);
                        let _ = stream
                        .send(rocket_ws::Message::Text(
                            serde_json::to_string(&msg).unwrap(),
                        )).await;
                    },
                    _ = &mut shutdown => {
                        let _ = stream
                        .send(rocket_ws::Message::Text(
                            serde_json::to_string(&Info::SystemShutdown).unwrap(),
                        )).await;
                        break;
                    },
                };
            }

            Ok(())

            // while let Ok(msg) = rx.recv().await {
            //     select! {

            //      _ ={
            //             }
            //     }
            // }

            // Ok(())
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
    info: &State<Sender<Info>>,
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

fn get_routes() -> Vec<Route> {
    routes![gen_info, get_fixture_types, add_fixture]
}

pub struct DataServingModule;

impl Module for DataServingModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        app.mount("/data", get_routes())
    }
}
