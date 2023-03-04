#[macro_use]
extern crate rocket;

mod paste_id;

use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::PathBuf;

use paste_id::PasteId;
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::response::content::RawText;
use rocket::serde::{json::Json, Serialize};
use rocket::tokio::fs::File;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

pub mod api;
pub mod views;

// In a real application, these would be retrieved dynamically from a config.
#[allow(clippy::declare_interior_mutable_const)]
const HOST: Absolute<'static> = uri!("https://gisty.shuttleapp.rs");

const ID_LENGTH: usize = 3;

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste
        .open(128.kibibytes())
        .into_file(id.file_path())
        .await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<RawText<File>> {
    File::open(id.file_path()).await.map(RawText).ok()
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GistFile {
    pub filename: String,
}

#[get("/all", format = "json")]
async fn all() -> Json<Vec<GistFile>> {
    let root: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let path: PathBuf = PathBuf::from(root);

    let mut files: Vec<GistFile> = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();
        if path.is_file() {
            let filename = path.file_stem().unwrap().to_str().unwrap().to_string();

            if filename.len() == ID_LENGTH {
                files.push(GistFile { filename });
            }
        }
    }

    Json(files)
}

#[delete("/<id>")]
async fn delete(id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).ok()
}

#[shuttle_service::main]
async fn rocket() -> Result<Rocket<Build>, shuttle_service::Error> {
    Ok(rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                views::home::index,
                upload,
                delete,
                retrieve,
                all,
                api::routes::docs::docs_route,
                api::routes::health::health_route
            ],
        )
        .mount("/api", routes![all]))
}
