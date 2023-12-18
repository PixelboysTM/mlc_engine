mod data_serving;
mod fixture;
mod project;
mod ui_serving;

use project::Project;
use rocket::launch;

#[launch]
async fn rocket() -> _ {
    let project = Project::default();
    if project.load("test").await.is_err() {
        let json = include_str!("../../led-nano-par.json");
        let fix = fixture::parse_ofl_fixture(json).unwrap();
        project
            .insert_fixture(
                fixture::parse_ofl_fixture(include_str!("../../led-par-56.json")).unwrap(),
            )
            .await;
        project.insert_fixture(fix).await;
        project.save_as("test").await.unwrap();
    }

    rocket::build()
        .manage(project)
        .mount("/", ui_serving::get_routes())
        .mount("/data", data_serving::get_routes())
}
