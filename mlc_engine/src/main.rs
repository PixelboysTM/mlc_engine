mod fixture;
mod project;
mod ui_serving;

use project::Project;
use rocket::launch;

#[launch]
async fn rocket() -> _ {
    let json = include_str!("../../led-nano-par.json");
    let fix = fixture::parse_ofl_fixture(json).unwrap();
    println!("{fix:#?}");

    let project = Project::default();
    if project.load("test").await.is_err() {
        project.save_as("test").await.unwrap();
    }

    rocket::build().mount("/", ui_serving::get_routes())
}
