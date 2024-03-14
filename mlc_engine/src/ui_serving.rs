use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use rocket::{fs::NamedFile, futures::lock::Mutex, get, Responder, Route, routes, State};
use rocket::response::Redirect;
use rocket_okapi::okapi::openapi3::{OpenApi, Responses};
use rocket_okapi::{JsonSchema, openapi, openapi_get_routes_spec, OpenApiFromRequest};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::util::add_default_response_schema;

use crate::module::Module;

// const OUT_PATH: &str = "out/";
const OUT_PATH: &str = "mlc_dioxus/dist/";

#[derive(Responder, JsonSchema)]
enum UiResponse {
    #[schemars(skip)]
    File(Option<NamedFile>),
    #[schemars(skip)]
    Redirect(Redirect),
}

impl OpenApiResponderInner for UiResponse {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut r = Responses::default();
        add_default_response_schema(&mut r, "misc", gen.json_schema::<UiResponse>());
        Ok(r)
    }
}

#[openapi(tag = "UI Serving")]
#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let r = NamedFile::open(Path::new(OUT_PATH).join(&file)).await.ok();
    if r.is_some() {
        r
    } else {
        let p = file.to_str().unwrap().to_string() + ".html";
        NamedFile::open(Path::new(OUT_PATH).join(p)).await.ok()
    }
}

#[openapi(tag = "UI Serving")]
#[get("/")]
async fn index(project_selection: &State<ProjectSelection>) -> UiResponse {
    if project_selection.inner().0.lock().await.is_some() {
        UiResponse::File(NamedFile::open(Path::new(OUT_PATH).join("index.html"))
            .await
            .ok())
    } else {
        UiResponse::Redirect(Redirect::to("/projects"))
        // NamedFile::open(Path::new(OUT_PATH).join("project.html"))
        //     .await
        //     .ok()
    }
}

#[openapi(tag = "UI Serving")]
#[get("/projects")]
async fn projects(project_selection: &State<ProjectSelection>) -> UiResponse {
    if project_selection.inner().0.lock().await.is_some() {
        // NamedFile::open(Path::new(OUT_PATH).join("index.html"))
        //     .await
        //     .ok()
        UiResponse::Redirect(Redirect::to("/"))
    } else {
        UiResponse::File(NamedFile::open(Path::new(OUT_PATH).join("index.html"))
            .await
            .ok())
    }
}

#[openapi(tag = "UI Serving")]
#[get("/viewer3d")]
async fn viewer3d(project_selection: &State<ProjectSelection>) -> Option<NamedFile> {
    if project_selection.inner().0.lock().await.is_some() {
        NamedFile::open(Path::new(OUT_PATH).join("viewer-3d.html"))
            .await
            .ok()
    } else {
        NamedFile::open(Path::new(OUT_PATH).join("error.html")) // TODO: Custom Error page
            .await
            .ok()
    }
}

fn get_routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![index, files, viewer3d, projects]
}

pub struct UiServingModule;

impl Module for UiServingModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>, spec: &mut OpenApi) -> rocket::Rocket<rocket::Build> {
        let (routes, s) = get_routes();
        merge_specs(spec, &"/".to_string(), &s).expect("Merging OpenApi failed");


        app.mount("/", routes)
            .manage(ProjectSelection(Arc::new(Mutex::new(None))))
    }
}

pub struct ProjectSelection(pub Arc<Mutex<Option<String>>>);
