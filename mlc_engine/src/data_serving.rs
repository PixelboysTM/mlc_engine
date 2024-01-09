use std::{str::FromStr, thread, time::Duration};

use rocket::{
    data::ToByteUnit,
    futures::SinkExt,
    get, post,
    response::status::{BadRequest, Custom},
    routes,
    serde::json::Json,
    tokio::{select, sync::broadcast::Sender},
    Data, Route, State,
};
use rocket_ws::WebSocket;
use uuid::Uuid;

use crate::{
    fixture::{self, UniverseId},
    module::Module,
    project::Project,
    settings::ProjectDefinition,
};

#[derive(serde::Serialize, Debug, Clone)]
pub enum Info {
    FixtureTypesUpdated,
    ProjectSaved,
    ProjectLoaded,
    SystemShutdown,
    UniversePatchChanged(UniverseId),
    UniversesUpdated,
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
        })
    })
}

#[derive(serde::Deserialize, serde::Serialize)]
struct FixtureInfo {
    name: String,
    id: uuid::Uuid,
    modes: Vec<String>,
}

#[get("/get/fixture-types")]
async fn get_fixture_types(project: &State<Project>) -> Json<Vec<FixtureInfo>> {
    Json(
        project
            .inner()
            .get_fixtures()
            .await
            .iter()
            .map(|f| FixtureInfo {
                id: *f.get_id(),
                name: f.get_name().to_string(),
                modes: f
                    .get_modes()
                    .iter()
                    .map(|m| m.get_name().to_string())
                    .collect(),
            })
            .collect::<Vec<FixtureInfo>>(),
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

    let fix = fixture::parse_fixture(&string).map_err(BadRequest)?;
    for fixture in fix {
        project.insert_fixture(fixture, info).await;
    }

    Ok(())
}

#[get("/universes")]
async fn get_universes(project: &State<Project>) -> Json<Vec<UniverseId>> {
    let data = project.get_universes().await;
    Json(data)
}

#[get("/save")]
async fn save_project(
    project: &State<Project>,
    info: &State<Sender<Info>>,
) -> Result<(), Custom<&'static str>> {
    project
        .save(info)
        .await
        .map_err(|e| Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/patch/<id>/<mode>?<create>")]
fn patch_fixture(
    project: &State<Project>,
    info: &State<Sender<Info>>,
    id: &str,
    mode: usize,
    create: bool,
) -> Result<Json<String>, String> {
    println!("{create}");
    let f_id = Uuid::from_str(id).map_err(|_| "Id is not valid".to_string())?;

    pollster::block_on(async {
        let fixture = project
            .get_fixtures()
            .await
            .iter()
            .find(|f| f.get_id() == &f_id)
            .cloned()
            .ok_or("Id is not a valid FixtureType".to_string())?;

        project
            .try_patch(&fixture, mode, create, info)
            .await
            .ok_or("Patching failed".to_owned())
            .map(|_| Json("Patching successful".to_string()))
    })
}

fn get_routes() -> Vec<Route> {
    routes![
        gen_info,
        get_fixture_types,
        add_fixture,
        save_project,
        get_universes,
        patch_fixture
    ]
}

pub struct DataServingModule;

impl Module for DataServingModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        app.mount("/data", get_routes())
    }
}
