use std::net::{IpAddr, Ipv4Addr};

use rocket::{
    catch, catchers, config::Ident, get, launch, serde::json::Json, tokio::sync::broadcast::Sender,
    Config, State,
};
use rocket_okapi::okapi::merge::merge_specs;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::rapidoc::{GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::SwaggerUIConfig;
use rocket_okapi::{openapi, openapi_get_routes_spec};

use data_serving::DataServingModule;
use mlc_common::Info;
use module::{Application, Module};
use project::ProjectHandle;
use runtime::RuntimeModule;
use settings::SettingsModule;
use ui_serving::UiServingModule;

mod data_serving;
mod fixture;
mod module;
mod project;
mod runtime;
mod settings;
mod ui_serving;
mod utils;

#[launch]
async fn rocket() -> _ {
    Application::create()
        .mount(MainModule)
        .mount(UiServingModule)
        .mount(DataServingModule)
        .mount(SettingsModule)
        .mount(RuntimeModule)
        .launch()
}

/// # Heartbeat
/// Is used to detect whether the backend is still running
///
/// Simply returns the Json of "alive" while available
#[openapi(tag = "Util")]
#[get("/heartbeat")]
async fn heart_beat() -> Json<&'static str> {
    Json("alive")
}

struct MainModule;

impl Module for MainModule {
    fn setup(
        &self,
        app: rocket::Rocket<rocket::Build>,
        spec: &mut OpenApi,
    ) -> rocket::Rocket<rocket::Build> {
        let (tx, rx) = rocket::tokio::sync::broadcast::channel::<Info>(100);

        let config = Config {
            address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            workers: 16,
            cli_colors: true,
            ident: Ident::try_new("Marvin Lighting Controller").unwrap(),

            ..Default::default()
        };

        let (routes, s) = openapi_get_routes_spec![heart_beat, create_empty];
        merge_specs(spec, &"/util".to_string(), &s).expect("Merging OpenApi spec failed");

        let p = pollster::block_on(async {
            let p = ProjectHandle::default();
            {
                let mut m = p.lock().await;
                m.settings.save_on_quit = false;
            }
            p
        });

        app.manage(p)
            // .attach(utils::BrowserGuard)
            .manage(tx)
            .manage(rx)
            .register("/", catchers![catch_404])
            .configure(config)
            .mount("/util", routes)
            .mount(
                "/api",
                rocket_okapi::swagger_ui::make_swagger_ui(&SwaggerUIConfig {
                    url: "../openapi.json".to_owned(),
                    ..Default::default()
                }),
            )
            .mount(
                "/rapi",
                rocket_okapi::rapidoc::make_rapidoc(&RapiDocConfig {
                    general: GeneralConfig {
                        spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                        ..Default::default()
                    },
                    hide_show: HideShowConfig {
                        allow_spec_file_load: false,
                        allow_spec_url_load: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
    }
}

#[catch(404)]
fn catch_404() -> &'static str {
    "Resource not available"
}

/// # Debug Create Project
/// Creates a default project and saves it to disk with the specified name.
///
/// Be careful we don't perform any checks or validations!
#[openapi(tag = "Util")]
#[get("/dCreate/<name>")]
async fn create_empty(name: &str, info: &State<Sender<Info>>) {
    let project = ProjectHandle::default();
    project.save_as(name, name, info).await.unwrap();
}
