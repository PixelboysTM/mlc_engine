use std::str::FromStr;

use rocket::{
    Data
    ,
    data::ToByteUnit,
    futures::SinkExt,
    get,
    http::Status,
    post,
    request::{self, FromRequest}
    ,
    Request,
    Responder,
    response::status::{BadRequest, Custom}, Route, serde::json::Json, State, tokio::{select, sync::broadcast::Sender},
};
use rocket_okapi::{JsonSchema, openapi, openapi_get_routes_spec, OpenApiFromRequest};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::okapi::openapi3::{OpenApi, Responses};
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::util::add_default_response_schema;
use rocket_ws::WebSocket;
use uuid::Uuid;

use mlc_common::{FixtureInfo, Info};
use mlc_common::patched::UniverseId;
use mlc_common::universe::FixtureUniverse;

use crate::{
    fixture::{self},
    module::Module,
    project::Project,
};
use crate::{runtime::RuntimeData, ui_serving::ProjectSelection};
use crate::fixture::Wrapper;

#[openapi(tag = "Data Serving")]
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
                }
                ;
            }

            Ok(())
        })
    })
}

#[openapi(tag = "Data Serving")]
#[get("/get/fixture-types")]
async fn get_fixture_types(project: &State<Project>, _g: ProjectGuard) -> Json<Vec<FixtureInfo>> {
    Json(
        project
            .inner()
            .get_fixtures()
            .await
            .iter()
            .map(|f| FixtureInfo {
                id: *f.get_id(),
                name: f.get_name().to_string(),
                modes: f.get_modes().to_vec(),
            })
            .collect::<Vec<FixtureInfo>>(),
    )
}

#[openapi(tag = "Data Serving")]
#[get("/add/fixture-ofl/<manufacturer>/<name>")]
async fn add_fixture_ofl(
    project: &State<Project>,
    info: &State<Sender<Info>>,
    manufacturer: &str,
    name: &str,
    _g: ProjectGuard,
) -> Result<(), BadRequest<String>> {
    let data = reqwest::get(format!(
        "https://open-fixture-library.org/{}/{}.aglight",
        manufacturer, name
    ))
        .await
        .map_err(|e| BadRequest(e.to_string()))?;
    let json = data.text().await.map_err(|e| BadRequest(e.to_string()))?;
    let fix = fixture::parse_fixture(&json).map_err(|e| BadRequest(e.to_string()))?;
    for fixture in fix {
        project.insert_fixture(fixture, info).await;
    }
    Ok(())
}

#[openapi(tag = "Data Serving")]
#[post("/add/fixture", data = "<data>")]
async fn add_fixture(
    data: Data<'_>,
    project: &State<Project>,
    info: &State<Sender<Info>>,
    _g: ProjectGuard,
) -> Result<(), BadRequest<String>> {
    let s = data.open(2.gibibytes());
    let string = s
        .into_string()
        .await
        .map_err(|_| BadRequest("Failed to read to string".to_string()))?;

    let fix = fixture::parse_fixture(&string).map_err(|e| {
        eprintln!("{}", e);
        BadRequest(e)
    })?;
    for fixture in fix {
        project.insert_fixture(fixture, info).await;
    }

    Ok(())
}

#[openapi(tag = "Data Serving")]
#[get("/universes")]
async fn get_universes(project: &State<Project>, _g: ProjectGuard) -> Json<Vec<UniverseId>> {
    let mut data = project.get_universes().await;
    data.sort();
    Json(data)
}

#[openapi(tag = "Data Serving")]
#[get("/universes/<id>")]
async fn get_universe(
    id: Wrapper,
    project: &State<Project>,
    _g: ProjectGuard,
) -> Json<Option<FixtureUniverse>> {
    let data = project.get_universe(&id).await;
    if let Ok(d) = data {
        Json(Some(d))
    } else {
        Json(None)
    }
}

#[openapi(tag = "Data Serving")]
#[get("/save")]
async fn save_project(
    project: &State<Project>,
    info: &State<Sender<Info>>,
    _g: ProjectGuard,
) -> Result<(), Custom<&'static str>> {
    project
        .save(info)
        .await
        .map_err(|e| Custom(rocket::http::Status::InternalServerError, e))
}

#[derive(Responder, JsonSchema)]
enum PatchResult {
    #[response(status = 400)]
    IdInvalid(String),

    #[response(status = 400)]
    ModeInvalid(String),

    #[response(status = 409)]
    Failed(String),

    #[response(status = 200)]
    Succsess(String),
}

impl OpenApiResponderInner for PatchResult {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut r = Responses::default();
        add_default_response_schema(&mut r, "text/plain", gen.json_schema::<PatchResult>());
        Ok(r)
    }
}

#[openapi(tag = "Data Serving")]
#[get("/patch/<id>/<mode>?<create>")]
fn patch_fixture(
    project: &State<Project>,
    info: &State<Sender<Info>>,
    runtime: &State<RuntimeData>,
    id: &str,
    mode: usize,
    create: bool,
    _g: ProjectGuard,
) -> PatchResult {
    println!("{create}");
    let f_id = Uuid::from_str(id); //.map_err(|_| "Id is not valid".to_string())?;
    if f_id.is_err() {
        return PatchResult::IdInvalid("Id is not valid".to_string());
    }

    pollster::block_on(async {
        let f_id = f_id.expect("Must be some");
        let fixture = project
            .get_fixtures()
            .await
            .iter()
            .find(|f| f.get_id() == &f_id)
            .cloned();
        // .ok_or("Id is not a valid FixtureType".to_string())?;
        if fixture.is_none() {
            return PatchResult::IdInvalid("Id is not a valid FixtureType".to_string());
        }

        let fixture = fixture.expect("Must be some");

        if mode >= fixture.get_modes().len() {
            return PatchResult::ModeInvalid("Mode is not available".to_string());
        }

        let r = project
            .try_patch(&fixture, mode, create, info, runtime)
            .await;
        if r.is_some() {
            return PatchResult::Succsess("Patching successful".to_string());
        }

        PatchResult::Failed("Patching failed".to_owned())
    })
}

fn get_routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        gen_info,
        get_fixture_types,
        add_fixture,
        add_fixture_ofl,
        save_project,
        get_universes,
        patch_fixture,
        get_universe
    ]
}

pub struct DataServingModule;

impl Module for DataServingModule {
    fn setup(
        &self,
        app: rocket::Rocket<rocket::Build>,
        spec: &mut OpenApi,
    ) -> rocket::Rocket<rocket::Build> {
        let (routes, s) = get_routes();
        merge_specs(spec, &"/data".to_string(), &s).expect("Merging OpenAPi failed");
        app.mount("/data", routes)
    }
}

#[derive(JsonSchema, OpenApiFromRequest)]
pub struct ProjectGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ProjectGuard {
    type Error = String;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let t = request.rocket().state::<ProjectSelection>().unwrap();
        if t.0.lock().await.is_some() {
            request::Outcome::Success(ProjectGuard)
        } else {
            request::Outcome::Error((Status::Unauthorized, "No project loaded!".to_string()))
        }
    }
}

// #[rocket::async_trait]
// impl Fairing for ProjectGuard {
//     fn info(&self) -> rocket::fairing::Info {
//         rocket::fairing::Info {
//             name: "Project Guard",
//             kind: Kind::Request,
//         }
//     }

//     async fn on_request<'life0, 'life1, 'life2, 'life3, 'life4>(
//         &'life0 self,
//         req: &'life1 mut rocket::Request<'life2>,
//         _data: &'life3 mut Data<'life4>,
//     ) where
//         'life0: 'async_trait,
//         'life1: 'async_trait,
//         'life2: 'async_trait,
//         'life3: 'async_trait,
//         'life4: 'async_trait,
//     {
//         if let Some(r) = &req.route() {
//             r.
//         }
//     }
// }
