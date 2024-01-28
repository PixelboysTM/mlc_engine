use chrono::{DateTime, Local};
use rocket::{
    fairing::{Fairing, Kind},
    get, post, routes,
    serde::json::Json,
    tokio::{fs, sync::broadcast::Sender},
    Route, State,
};

use crate::{
    data_serving::Info,
    module::Module,
    project::{self, Project, Settings},
    runtime::RuntimeData,
    ui_serving::ProjectSelection,
};

#[get("/get")]
async fn get_settings(project: &State<Project>) -> Result<Json<Settings>, String> {
    let settings = project.get_settings().await;
    Ok(Json(settings))
}

#[post("/update", data = "<settings>")]
async fn update_settings(
    project: &State<Project>,
    settings: Json<Settings>,
) -> Result<String, String> {
    project
        .update_settings(settings.0)
        .await
        .map(|_| "Settings successfully updated".to_string())
        .map_err(|e| e.to_string())
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ProjectDefinition {
    pub name: String,
    #[serde(default)]
    pub file_name: String,
    pub last_edited: DateTime<Local>,
}

#[get("/projects-list")]
async fn get_available_projects() -> Json<Vec<ProjectDefinition>> {
    let path = project::make_path("test")
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();

    let mut projects = vec![];
    let iter = std::fs::read_dir(path).unwrap();
    for f in iter.flatten() {
        if f.file_type().unwrap().is_file() && f.file_name().to_string_lossy().ends_with(".mlc") {
            let data = fs::read_to_string(f.path()).await.unwrap();
            let mut defintition: ProjectDefinition = serde_json::from_str(&data).unwrap();
            defintition.file_name = f.file_name().to_string_lossy().replace(".mlc", "");
            projects.push(defintition);
        }
    }

    Json(projects)
}

#[get("/load/<name>")]
async fn load_project(
    name: &str,
    project: &State<Project>,
    project_selection: &State<ProjectSelection>,
    info: &State<Sender<Info>>,
    runtime: &State<RuntimeData>,
) -> Result<String, String> {
    if project_selection.0.lock().await.is_some() {
        return Err("Project already loaded why on this page.".to_string());
    }

    let result = project.load(name, info.inner(), runtime).await;
    if result.is_err() {
        eprintln!("{:?}", result.unwrap_err());
        return result.map_err(|e| e.to_string()).map(|_| "".to_string());
    }
    let mut p = project_selection.0.lock().await;
    *p = Some(name.to_string());

    Ok("Loaded succsessful".to_string())
}

#[get("/current")]
async fn get_current_project(project: &State<Project>) -> Json<ProjectDefinition> {
    Json(project.get_definition().await)
}

fn get_routes() -> Vec<Route> {
    routes![get_settings, update_settings]
}

pub struct SettingsModule;

impl Module for SettingsModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        app.mount("/settings", get_routes())
            .attach(ShutdownSaver)
            .mount(
                "/projects",
                routes![get_available_projects, load_project, get_current_project],
            )
    }
}

struct ShutdownSaver;

impl Fairing for ShutdownSaver {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            kind: Kind::Shutdown,
            name: "ShutdownSaver",
        }
    }

    fn on_shutdown<'life0, 'life1, 'async_trait>(
        &'life0 self,
        rocket: &'life1 rocket::Rocket<rocket::Orbit>,
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ()> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let project: Option<&Project> = rocket.state();
            if let Some(p) = project {
                if p.get_settings().await.save_on_quit() {
                    p.save(rocket.state().unwrap()).await.unwrap();
                }
            }
        })
    }
}
