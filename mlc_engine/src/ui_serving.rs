use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use rocket::{fs::NamedFile, futures::lock::Mutex, get, Responder, Route, routes, State};
use rocket::response::Redirect;

use crate::module::Module;

// const OUT_PATH: &str = "out/";
const OUT_PATH: &str = "mlc_dioxus/dist/";

#[derive(Responder)]
enum UiResponse {
    File(Option<NamedFile>),
    Redirect(Redirect),
}

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

fn get_routes() -> Vec<Route> {
    routes![index, files, viewer3d, projects]
}

pub struct UiServingModule;

impl Module for UiServingModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        app.mount("/", get_routes())
            .manage(ProjectSelection(Arc::new(Mutex::new(None))))
    }
}

pub struct ProjectSelection(pub Arc<Mutex<Option<String>>>);
