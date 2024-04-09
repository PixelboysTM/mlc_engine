use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use rocket::response::Redirect;
use rocket::{fs::NamedFile, futures::lock::Mutex, get, Responder, Route, State};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::okapi::openapi3::{OpenApi, Responses};
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::util::add_default_response_schema;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};

use crate::module::Module;

const OUT_PATH: &str = "out/";
// const OUT_PATH: &str = "mlc_dioxus/dist/";

#[derive(Responder, JsonSchema)]
#[allow(variant_size_differences, clippy::large_enum_variant)]
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

/// # File serving
/// Route that serves all Misc files needed for the ui to work such as js, WebAssembly, CSS, etc.
///
/// Note: File extension can be omitted for html files. (GET /some-random-html.html -> GET /some-random-html)
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

/// # Index
/// Checks whether a valid Project is loaded and if so continues to serve the normal MLC UI.
/// If no valid Project is loaded a redirect to the Project Browser is returned.
#[openapi(tag = "UI Serving")]
#[get("/")]
async fn index(project_selection: &State<ProjectSelection>) -> UiResponse {
    if project_selection.inner().0.lock().await.is_some() {
        UiResponse::File(
            NamedFile::open(Path::new(OUT_PATH).join("index.html"))
                .await
                .ok(),
        )
    } else {
        UiResponse::Redirect(Redirect::to("/projects"))
        // NamedFile::open(Path::new(OUT_PATH).join("project.html"))
        //     .await
        //     .ok()
    }
}

/// # Project Browser
/// Checks whether a valid Project is loaded and when **not** so displays the Project Browser UI.
/// If a valid Project is loaded a redirect to the Index is returned.
#[openapi(tag = "UI Serving")]
#[get("/projects")]
async fn projects(project_selection: &State<ProjectSelection>) -> UiResponse {
    if project_selection.inner().0.lock().await.is_some() {
        // NamedFile::open(Path::new(OUT_PATH).join("index.html"))
        //     .await
        //     .ok()
        UiResponse::Redirect(Redirect::to("/"))
    } else {
        UiResponse::File(
            NamedFile::open(Path::new(OUT_PATH).join("index.html"))
                .await
                .ok(),
        )
    }
}

/// # 3D Viewer
/// Checks whether a valid project is loaded and if so displays the 3D Viewer in a seperate window.
/// Otherwise, returns an Error page.
#[openapi(tag = "UI Serving")]
#[get("/viewer")]
async fn viewer3d(project_selection: &State<ProjectSelection>) -> Option<NamedFile> {
    NamedFile::open(Path::new(OUT_PATH).join("index.html")).await.ok()
    // if project_selection.inner().0.lock().await.is_some() {
    //     NamedFile::open(Path::new(OUT_PATH).join("viewer.html"))
    //         .await
    //         .ok()
    // } else {
    //     NamedFile::open(Path::new(OUT_PATH).join("error.html")) // TODO: Custom Error page
    //         .await
    //         .ok()
    // }
}

fn get_routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![index, files, viewer3d, projects]
}

pub struct UiServingModule;

impl Module for UiServingModule {
    fn setup(
        &self,
        app: rocket::Rocket<rocket::Build>,
        spec: &mut OpenApi,
    ) -> rocket::Rocket<rocket::Build> {
        let (routes, s) = get_routes();
        merge_specs(spec, &"/".to_string(), &s).expect("Merging OpenApi failed");

        app.mount("/", routes)
            .manage(ProjectSelection(Arc::new(Mutex::new(None))))
    }
}

pub struct ProjectSelection(pub Arc<Mutex<Option<String>>>);
