use rocket::{
    fairing::{Fairing, Kind},
    get, post,
    serde::json::Json,
    tokio::{fs, sync::broadcast::Sender},
    Route, State,
};
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::{openapi, openapi_get_routes_spec};

use mlc_common::{CreateProjectData, Info, ProjectDefinition, ProjectSettings};

use crate::{
    data_serving::ProjectGuard,
    module::Module,
    project::{self, ProjectHandle},
    runtime::RuntimeData,
    send,
    ui_serving::ProjectSelection,
};
use crate::{project::Provider, runtime::effects::player::EffectPlayerHandle};

/// # Get Settings
/// Returns the current project settings
///
/// [Guarded][`ProjectGuard`]
#[openapi(tag = "Settings")]
#[get("/get")]
async fn get_settings(
    project: &State<ProjectHandle>,
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
    project: &State<ProjectHandle>,
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

    let path = project::make_path("test", None)
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
            definition.file_name = f
                .path()
                .file_name()
                .expect("Why no Filename?")
                .to_string_lossy()
                .to_string();
            definition.binary = ext.is_binary();
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
    project: &State<ProjectHandle>,
    project_selection: &State<ProjectSelection>,
    info: &State<Sender<Info>>,
    runtime: &State<RuntimeData>,
    effect_handler: &State<EffectPlayerHandle>,
) -> Result<Json<String>, String> {
    if project_selection.0.lock().await.is_some() {
        return Err("Project already loaded why on this page.".to_string());
    }

    let result = project
        .load(
            name,
            info.inner(),
            runtime,
            &mut effect_handler.inner().clone().cmd_sender,
        )
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
    project: &State<ProjectHandle>,
    _g: ProjectGuard,
) -> Json<ProjectDefinition> {
    Json(project.get_definition().await)
}

/// # Close Project
/// Closes the current project to return to project selection.
///
/// Saves the project if "Save on close" is turned on.
///
/// [Guarded][`ProjectGuard`]
#[openapi(ignore = "_g", tag = "Projects")]
#[get("/close")]
async fn close_project(
    project: &State<ProjectHandle>,
    info: &State<Sender<Info>>,
    project_selection: &State<ProjectSelection>,
    _g: ProjectGuard,
) -> Result<(), String> {
    if project.get_settings().await.save_on_quit {
        project.save(info).await.map_err(|e| e.to_string())?;
    }

    project.close().await;
    let mut l = project_selection.0.lock().await;
    *l = None;
    send!(info, Info::RequireReload);

    Ok(())
}

/// # Create New Project
/// If Currently no project is loaded creates a new project with the given configuratzion and loads it.
#[openapi(tag = "Projects")]
#[post("/create", data = "<data>")]
async fn create_project(
    data: Json<CreateProjectData>,
    info: &State<Sender<Info>>,
    runtime: &State<RuntimeData>,
    effect_handler: &State<EffectPlayerHandle>,
    project: &State<ProjectHandle>,
    project_selection: &State<ProjectSelection>,
) -> Result<Json<String>, Json<String>> {
    if project_selection.0.lock().await.is_some() {
        return Err(Json(
            "Can't create projects while a project is loaded!".to_string(),
        ));
    }

    let name = data.name.clone();
    let binary = data.binary;
    let file_name = format!(
        "{}.{}",
        mlc_common::to_save_file_name(&name),
        if binary {
            Provider::Ciborium
        } else {
            Provider::Json
        }
        .extension()
    );

    let new_project = ProjectHandle::default();
    new_project
        .save_as(&name, &file_name, info)
        .await
        .map_err(|e| Json(e.to_string()))?;

    project
        .load(
            &file_name,
            info,
            runtime,
            &mut effect_handler.inner().clone().cmd_sender,
        )
        .await
        .map_err(|e| Json(e.to_string()))?;
    let mut p = project_selection.0.lock().await;
    *p = Some(name.to_string());

    Ok(Json("Loaded successful".to_string()))
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
        let (routes, s) = openapi_get_routes_spec![
            get_available_projects,
            load_project,
            get_current_project,
            close_project,
            create_project
        ];
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
        Box<dyn core::future::Future<Output = ()> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let project: Option<&ProjectHandle> = rocket.state();
            if let Some(p) = project {
                if p.get_settings().await.save_on_quit {
                    p.save(rocket.state().unwrap()).await.unwrap();
                }
            }
        })
    }
}
