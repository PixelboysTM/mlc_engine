use std::str::FromStr;

use rocket::{
    data::ToByteUnit,
    futures::SinkExt,
    get,
    http::Status,
    post,
    request::{self, FromRequest},
    response::status::{BadRequest, Custom},
    serde::json::Json,
    tokio::{select, sync::broadcast::Sender},
    Data, Request, Responder, Route, State,
};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::okapi::openapi3::{OpenApi, Responses};
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::util::add_default_response_schema;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema, OpenApiFromRequest};
use rocket_ws::WebSocket;
use uuid::Uuid;

use mlc_common::patched::feature::{FixtureFeatureType, HasFixtureFeature};
use mlc_common::patched::UniverseId;
use mlc_common::universe::FixtureUniverse;
use mlc_common::{FixtureInfo, Info};

use crate::fixture::UniverseIdParam;
use crate::{
    fixture::{self},
    module::Module,
    project::ProjectHandle,
};
use crate::{runtime::RuntimeData, ui_serving::ProjectSelection};

/// # Info
/// Upgrades to a WebSocket on which general Information can be received.
/// For more information and a List on messages that can be received see [`mlc_common::Info`]
#[openapi(tag = "Data Serving")]
#[get("/info")]
pub async fn gen_info(
    ws: WebSocket,
    tx: &State<Sender<Info>>,
    mut shutdown: rocket::Shutdown,
) -> rocket_ws::Channel {
    let mut rx = tx.subscribe();
    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    Ok(msg) = rx.recv() => {
                        // println!("{:?}", msg);
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
            }

            Ok(())
        })
    })
}

/// # Get Fixture Types
/// Returns a list of all FixtureTypes in the current project.
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Data Serving")]
#[get("/get/fixture-types")]
async fn get_fixture_types(
    project: &State<ProjectHandle>,
    _g: ProjectGuard,
) -> Json<Vec<FixtureInfo>> {
    Json(
        project
            .inner()
            .get_fixtures()
            .await
            .iter()
            .map(|f| FixtureInfo {
                id: f.id,
                name: f.name.clone(),
                modes: f.get_modes().to_vec(),
            })
            .collect::<Vec<FixtureInfo>>(),
    )
}

/// # Add Fixture Ofl
/// Add a new Fixture Type by querying Json from http://open-fixture-library.org/
///
/// On Success: Nothing is returned
///
/// On Failure: A Bad Request is returned with an error String.
///
/// Note: The MLC host must be connected to the internet
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Data Serving")]
#[get("/add/fixture-ofl/<manufacturer>/<name>")]
async fn add_fixture_ofl(
    project: &State<ProjectHandle>,
    info: &State<Sender<Info>>,
    manufacturer: &str,
    name: &str,
    _g: ProjectGuard,
) -> Result<(), BadRequest<String>> {
    let data = reqwest::get(format!(
        "https://open-fixture-library.org/{}/{}.json",
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

/// # Add Fixture
/// Add a new Fixture Type using raw OFL Json
///
/// On Success: Nothing is returned
///
/// On Failure: BadRequest is returned with an error String
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Data Serving")]
#[post("/add/fixture", data = "<data>")]
async fn add_fixture(
    data: Data<'_>,
    project: &State<ProjectHandle>,
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

/// # Get universes
/// Returns a List of Universe Ids. Normally this will be a List of ascending Integers starting at 1
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Data Serving")]
#[get("/universes")]
async fn get_universes(project: &State<ProjectHandle>, _g: ProjectGuard) -> Json<Vec<UniverseId>> {
    let mut data = project.get_universes().await;
    data.sort();
    Json(data)
}

/// # Get Universe
/// Returns the [`mlc_common::FixtureUniverse`] of the requested UniverseId.
///
/// Returns an empty Json when the UniverseId is not valid in the current Project
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Data Serving")]
#[get("/universes/<id>")]
async fn get_universe(
    id: UniverseIdParam,
    project: &State<ProjectHandle>,
    _g: ProjectGuard,
) -> Json<Option<FixtureUniverse>> {
    let data = project.get_universe(&id).await;
    if let Ok(d) = data {
        Json(Some(d))
    } else {
        Json(None)
    }
}

/// # Save
/// Saves the current project
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Data Serving")]
#[get("/save")]
async fn save_project(
    project: &State<ProjectHandle>,
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
    Success(String),
}

impl OpenApiResponderInner for PatchResult {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut r = Responses::default();
        add_default_response_schema(&mut r, "text/plain", gen.json_schema::<PatchResult>());
        Ok(r)
    }
}

/// # Patch Fixture
/// Will be extended and Documented when done so
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Data Serving")]
#[get("/patch/<id>/<mode>?<create>")]
fn patch_fixture(
    project: &State<ProjectHandle>,
    info: &State<Sender<Info>>,
    runtime: &State<RuntimeData>,
    id: &str,
    mode: usize,
    create: bool,
    _g: ProjectGuard,
) -> PatchResult {
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
            .find(|f| f.id == f_id)
            .cloned();
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
            return PatchResult::Success("Patching successful".to_string());
        }

        PatchResult::Failed("Patching failed".to_owned())
    })
}

/// # Features by Fixture
/// Returns a list of Tuples containing each patched fixture and its available FixtureFeatureTypes
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Data Serving")]
#[get("/all_features")]
async fn get_features_by_fixtures(
    project: &State<ProjectHandle>,
    _g: ProjectGuard,
) -> Json<Vec<(uuid::Uuid, Vec<FixtureFeatureType>)>> {
    let us = project.get_universes().await;
    let mut result = vec![];
    for u in us {
        let universe = project.get_universe(&u).await.unwrap();
        let mut fixtures = universe
            .fixtures
            .iter()
            .map(|i| {
                (
                    i.id,
                    i.features.iter().map(|f| f.name()).collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        result.append(&mut fixtures);
    }

    Json(result)
}

/// # All Fixtures with Feature
/// Returns all Fixtures that have the specified Feature
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Data Serving")]
#[get("/all_with_feature/<feature_name>")]
async fn all_with_feature(
    project: &State<ProjectHandle>,
    _g: ProjectGuard,
    feature_name: String,
) -> Result<Json<Vec<(Uuid, String)>>, String> {
    let feature_name = feature_name.parse::<FixtureFeatureType>()?;
    let us = project.get_universes().await;
    let mut result = vec![];
    for u in us {
        let universe = project.get_universe(&u).await.unwrap();
        let mut fixtures = universe
            .fixtures
            .iter()
            .filter(|f| f.features.has(&feature_name))
            .map(|f| (f.id, f.name.clone()))
            .collect::<Vec<_>>();
        result.append(&mut fixtures);
    }

    Ok(Json(result))
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
        get_universe,
        get_features_by_fixtures,
        all_with_feature
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

/// # Project Guard
/// When used in a request as a Parameter checks whether a valid Project is loaded. If **not** so the Request is cancelled with an error.
///
/// Use this to ensure a valid project is loaded before project data is changed or queried.
///
/// Note: This does not Provide you with the Project itself, for that you need to Request a separate State `&State<Project>`. This Guard only guarantees you that you can safely use the Project provided.
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
