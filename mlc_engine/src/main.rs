use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, get, launch, routes};

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("mlc_web/dist/").join(file))
        .await
        .ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("mlc_web/dist/index.html"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
