use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, get, routes, Route};

const OUT_PATH: &str = "out/";

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let r = NamedFile::open(Path::new(OUT_PATH).join(&file)).await.ok();
    if r.is_some() {
        r
    } else {
        let p = file.to_str().unwrap().to_string() + ".html";
        NamedFile::open(Path::new(OUT_PATH).join(p)).await.ok()
    }
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(OUT_PATH).join("index.html"))
        .await
        .ok()
}

pub fn get_routes() -> Vec<Route> {
    routes![index, files]
}
