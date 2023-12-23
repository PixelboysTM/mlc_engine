use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use rocket::{fs::NamedFile, futures::lock::Mutex, get, routes, Route, State};

use crate::module::Module;

const OUT_PATH: &str = "out/";

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
async fn index(project_selection: &State<ProjectSelection>) -> Option<NamedFile> {
    if project_selection.inner().0.lock().await.is_some() {
        NamedFile::open(Path::new(OUT_PATH).join("index.html"))
            .await
            .ok()
    } else {
        NamedFile::open(Path::new(OUT_PATH).join("project.html"))
            .await
            .ok()
    }
}

fn get_routes() -> Vec<Route> {
    routes![index, files]
}

pub struct UiServingModule;

impl Module for UiServingModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        app.mount("/", get_routes())
            .manage(ProjectSelection(Arc::new(Mutex::new(None))))
    }
}

pub struct ProjectSelection(Arc<Mutex<Option<String>>>);
