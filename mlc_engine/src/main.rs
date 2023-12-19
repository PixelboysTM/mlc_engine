mod data_serving;
mod data_spreader;
mod fixture;
mod project;
mod side_worker;
mod ui_serving;

use data_serving::Info;
use data_spreader::DataSender;
use project::Project;
use rocket::{get, launch, routes, State};

#[launch]
async fn rocket() -> _ {
    // let (info_rx, info_tx) = data_serving::create_info();

    let side_worker = side_worker::create_side_worker();
    // side_worker.queue_job(Box::new(Hello(u32::MAX))).await;

    let (w, s, r) = data_spreader::create::<Info, uuid::Uuid>();
    side_worker.queue_job(Box::new(w)).await;

    let project = Project::default();
    if project.load("test", &s).await.is_err() {
        let json = include_str!("../../led-nano-par.json");
        let fix = fixture::parse_ofl_fixture(json).unwrap();
        project
            .insert_fixture(
                fixture::parse_ofl_fixture(include_str!("../../led-par-56.json")).unwrap(),
                &s,
            )
            .await;
        project.insert_fixture(fix, &s).await;
        project.save_as("test", &s).await.unwrap();
    }

    rocket::build()
        .manage(project)
        .manage(side_worker)
        .manage(r)
        .manage(s)
        .mount("/", ui_serving::get_routes())
        .mount("/data", data_serving::get_routes())
        .mount("/api", routes![send_msg])
}

#[get("/")]
fn send_msg(d: &State<DataSender<Info>>) {
    d.send(Info::ProjectSaved);
}

struct Hello(u32);
impl side_worker::Work for Hello {
    fn run(&mut self) -> bool {
        if self.0 > 0 {
            println!("Hello");
            self.0 -= 1;
            true
        } else {
            false
        }
    }
}
