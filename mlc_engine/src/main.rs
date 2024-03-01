mod data_serving;
mod fixture;
mod module;
mod project;
mod runtime;
mod settings;
mod ui_serving;
mod utils;

use std::net::{IpAddr, Ipv4Addr};

use data_serving::{DataServingModule};
use module::{Application, Module};
use project::Project;
use rocket::{
    catch, catchers, config::Ident, get, launch, routes, serde::json::Json,
    tokio::sync::broadcast::Sender, Config, State,
};
use mlc_common::Info;
use runtime::RuntimeModule;
use settings::SettingsModule;
use ui_serving::UiServingModule;

#[launch]
async fn rocket() -> _ {
    Application::create()
        .mount(MainModule)
        .mount(UiServingModule)
        .mount(DataServingModule)
        .mount(SettingsModule)
        .mount(RuntimeModule)
        .launch()
    // rocket::build()
    //     .manage(project)
    //     .manage(tx)
    //     .manage(rx)
    //     .mount("/", ui_serving::get_routes())
    //     .mount("/data", data_serving::get_routes())
}

struct MainModule;

#[get("/heartbeat")]
async fn heart_beat() -> Json<&'static str> {
    Json("alive")
}

impl Module for MainModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        let (tx, rx) = rocket::tokio::sync::broadcast::channel::<Info>(100);

        let config = Config {
            address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            cli_colors: true,
            ident: Ident::try_new("Marvin Lighting Controller").unwrap(),

            ..Default::default()
        };

        app.manage(Project::default())
            .manage(tx)
            .manage(rx)
            .register("/", catchers![catch_404])
            .configure(config)
            .mount("/util", routes![heart_beat, create_empty])
    }
}

#[catch(404)]
fn catch_404() -> &'static str {
    "Resource not available"
}

#[get("/dCreate/<name>")]
async fn create_empty(name: &str, info: &State<Sender<Info>>) {
    let project = Project::default();
    project.save_as(name, info).await.unwrap();
}
