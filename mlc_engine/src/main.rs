mod data_serving;
mod fixture;
mod module;
mod project;
mod ui_serving;
mod utils;

use std::net::{IpAddr, Ipv4Addr};

use data_serving::{DataServingModule, Info};
use module::{Application, Module};
use project::Project;
use rocket::{config::Ident, launch, Config};
use ui_serving::UiServingModule;

#[launch]
async fn rocket() -> _ {
    Application::create()
        .mount(MainModule)
        .mount(UiServingModule)
        .mount(DataServingModule)
        .launch()
    // rocket::build()
    //     .manage(project)
    //     .manage(tx)
    //     .manage(rx)
    //     .mount("/", ui_serving::get_routes())
    //     .mount("/data", data_serving::get_routes())
}

struct MainModule;

impl Module for MainModule {
    fn setup(&self, app: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        let (tx, rx) = rocket::tokio::sync::broadcast::channel::<Info>(100);

        let project = pollster::block_on(async {
            let project = Project::default();
            if project.load("test", &tx).await.is_err() {
                let json = include_str!("../../test_fixtures/led_par_56.json");
                let fix = fixture::parse_fixture(json).unwrap();
                // project
                //     .insert_fixture(
                //         fixture::parse_fixture(include_str!("../../led-par-56.json")).unwrap(),
                //         &tx,
                //     )
                //     .await;
                project.insert_fixture(fix[0].clone(), &tx).await;
                project.save_as("test", &tx).await.unwrap();
            }

            project
        });

        let config = Config {
            address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            cli_colors: true,
            ident: Ident::try_new("Marvin Lighting Controller").unwrap(),
            ..Default::default()
        };

        app.manage(project).manage(tx).manage(rx).configure(config)
    }
}
