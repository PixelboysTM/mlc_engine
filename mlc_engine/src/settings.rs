use rocket::{
    fairing::{Fairing, Kind},
    get, post,
    Route,
    serde::json::Json,
    State, tokio::{fs, sync::broadcast::Sender},
};
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::okapi::openapi3::OpenApi;

use mlc_common::{Info, ProjectDefinition, ProjectSettings};

use crate::{
    data_serving::ProjectGuard,
    module::Module,
    project::{self, Project},
    runtime::{effects::EffectPlayerAction, RuntimeData},
    ui_serving::ProjectSelection,
};
use crate::project::Provider;

/// # Get Settings
/// Returns the current project settings
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Settings")]
#[get("/get")]
async fn get_settings(
    project: &State<Project>,
    _g: ProjectGuard,
) -> Result<Json<ProjectSettings>, String> {
    let settings = project.get_settings().await;
    Ok(Json(settings))
}

/// # Update Settings
/// Updates the current Project settings with settings Json provided in the body
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Settings")]
#[post("/update", data = "<settings>")]
async fn update_settings(
    project: &State<Project>,
    settings: Json<ProjectSettings>,
    _g: ProjectGuard,
) -> Result<Json<String>, String> {
    project
        .update_settings(settings.0)
        .await
        .map(|_| Json("Settings successfully updated".to_string()))
        .map_err(|e| e.to_string())
}

/// # List Projects
/// Returns a [`mlc_common::ProjectDefinition`] List of all available Projects.
///
/// Updates available Projects fresh from files on disk each time
#[openapi(tag = "Projects")]
#[get("/projects-list")]
async fn get_available_projects() -> Json<Vec<ProjectDefinition>> {
    fn is_valid(f: &str) -> Option<Provider> {
        for (extension, p) in Provider::valid_extensions() {
            if f.ends_with(extension) {
                return Some(p);
            }
        }

        None
    }

    let path = project::make_path("test", "mlc")
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();

    let mut projects = vec![];
    let iter = std::fs::read_dir(path).unwrap();
    for f in iter.flatten() {
        let ext = is_valid(f.file_name().to_string_lossy().as_ref());
        if f.file_type().unwrap().is_file() && ext.is_some() {
            let ext = ext.expect("Tested before");
            let data = fs::read(f.path()).await.unwrap();
            let mut definition: ProjectDefinition = ext.definition(&data).unwrap();
            definition.file_name = f.file_name().to_string_lossy().replace(&format!(".{}", ext.extension()), "");

            // let data = fs::read_to_string(f.path()).await.unwrap();
            // let mut defintition: ProjectDefinition = serde_json::from_str(&data).unwrap();
            // defintition.file_name = f.file_name().to_string_lossy().replace(".mlc", "");
            projects.push(definition);
        }
    }

    Json(projects)
}

/// # Load Project
/// When no project is currently loaded the project with the specified name is loaded.
///
/// Else an error String is returned.
///
/// TODO: Make into async EventStream for loading progress
#[openapi(tag = "Projects")]
#[get("/load/<name>")]
async fn load_project(
    name: &str,
    project: &State<Project>,
    project_selection: &State<ProjectSelection>,
    info: &State<Sender<Info>>,
    runtime: &State<RuntimeData>,
    effect_handler: &State<Sender<EffectPlayerAction>>,
) -> Result<Json<String>, String> {
    if project_selection.0.lock().await.is_some() {
        return Err("Project already loaded why on this page.".to_string());
    }

    let result = project
        .load(name, info.inner(), runtime, effect_handler)
        .await;
    if result.is_err() {
        eprintln!("{:?}", result.unwrap_err());
        return result
            .map_err(|e| e.to_string())
            .map(|_| Json("".to_string()));
    }
    let mut p = project_selection.0.lock().await;
    *p = Some(name.to_string());

    Ok(Json("Loaded succsessful".to_string()))
}

/// # Get current Project
/// Returns the [`mlc_common::ProjectDefinition`] of the current loaded project.
///
/// Note: Definition is derived from Project and not read from disk.
///
/// [Guarded][`ProjectGuard`]
#[openapi(ignore = "_g", tag = "Projects")]
#[get("/current")]
async fn get_current_project(
    project: &State<Project>,
    _g: ProjectGuard,
) -> Json<ProjectDefinition> {
    Json(project.get_definition().await)
}

fn get_routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![get_settings, update_settings]
}

pub struct SettingsModule;

impl Module for SettingsModule {
    fn setup(
        &self,
        app: rocket::Rocket<rocket::Build>,
        spec: &mut OpenApi,
    ) -> rocket::Rocket<rocket::Build> {
        let (routes, s) =
            openapi_get_routes_spec![get_available_projects, load_project, get_current_project];
        merge_specs(spec, &"/projects".to_string(), &s).expect("Merging OpenApi failed");
        let (routes2, s2) = get_routes();
        merge_specs(spec, &"/settings".to_string(), &s2).expect("Merging OpenApi failed");
        app.mount("/settings", routes2)
            .attach(ShutdownSaver)
            .mount("/projects", routes)
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
        Box<dyn core::future::Future<Output=()> + core::marker::Send + 'async_trait>,
    >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
    {
        Box::pin(async {
            let project: Option<&Project> = rocket.state();
            if let Some(p) = project {
                if p.get_settings().await.save_on_quit {
                    p.save(rocket.state().unwrap()).await.unwrap();
                }
            }
        })
    }
}
