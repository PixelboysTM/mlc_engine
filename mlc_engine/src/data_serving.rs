use std::{thread, time::Duration};

use rocket::{get, response::content::RawJson, routes, Route, State};
use rocket_ws::WebSocket;

use crate::project::Project;

#[get("/info")]
async fn gen_info(ws: WebSocket) -> rocket_ws::Stream![] {
    rocket_ws::Stream! {ws =>
        for await msg in ws {
            yield msg?;
        }
    }
}

#[get("/get/fixture-types")]
async fn get_fixture_types(project: &State<Project>) -> RawJson<String> {
    thread::sleep(Duration::new(1, 0));

    RawJson(
        serde_json::to_string(
            &project
                .inner()
                .get_fixtures()
                .await
                .iter()
                .map(|f| f.get_name())
                .collect::<Vec<&str>>(),
        )
        .unwrap(),
    )
}

pub fn get_routes() -> Vec<Route> {
    routes![gen_info, get_fixture_types]
}
