mod data_serving;
mod fixture;
mod project;
mod ui_serving;

use project::Project;
use rocket::launch;

#[launch]
async fn rocket() -> _ {
    let (info_rx, info_tx) = data_serving::create_info();

    let project = Project::default();
    if project.load("test", &info_tx).await.is_err() {
        let json = include_str!("../../led-nano-par.json");
        let fix = fixture::parse_ofl_fixture(json).unwrap();
        project
            .insert_fixture(
                fixture::parse_ofl_fixture(include_str!("../../led-par-56.json")).unwrap(),
                &info_tx,
            )
            .await;
        project.insert_fixture(fix, &info_tx).await;
        project.save_as("test", &info_tx).await.unwrap();
    }

    rocket::build()
        .manage(project)
        .manage(info_rx)
        .manage(info_tx)
        .mount("/", ui_serving::get_routes())
        .mount("/data", data_serving::get_routes())
}
