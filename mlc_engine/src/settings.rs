use rocket::{
    fairing::{AdHoc, Fairing, Info, Kind},
    get, routes,
    serde::json::Json,
    Route, State,
};

use crate::{
    module::Module,
    project::{Project, Settings},
};

#[get("/get/settings")]
async fn get_settings(project: &State<Project>) -> Result<Json<Settings>, String> {
    let settings = project.get_settings().await;
    Ok(Json(settings))
}

#[get("/update/settings", data = "<settings>")]
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

fn get_routes() -> Vec<Route> {
    routes![get_settings, update_settings]
}

pub struct SettingsModule;

impl Module for SettingsModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        app.mount("/settings", get_routes()).attach(ShutdownSaver)
    }
}

struct ShutdownSaver;

impl Fairing for ShutdownSaver {
    fn info(&self) -> Info {
        Info {
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
