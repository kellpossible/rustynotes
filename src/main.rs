#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate askama;

#[macro_use]
extern crate rust_embed;

use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;
use std::io::Cursor;
use rocket::Response;
use rocket::response;
use rocket::http::{ContentType};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAsset;

#[get("/")]
fn index() -> IndexTemplate<'static> {
    IndexTemplate { name: "Luke" }
}

#[get("/static/<file..>")]
fn static_files<'r>(file: PathBuf) -> Result<Response<'r>, response::status::NotFound<String>> {
    let filename = file.display().to_string();

    let cloned_file = file.clone();
    let ext = cloned_file
        .as_path()
        .extension()
        .and_then(OsStr::to_str)
        .expect("Could not get file extension");

    let content_type = ContentType::from_extension(ext).expect("Could not get file content type");

    let path = Path::new("static/").join(file);

    println!("{:?}", filename);
    println!("{:?}", path);

    let asset = StaticAsset::get(&filename).unwrap();
    Response::build()
        .header(content_type)
        .sized_body(Cursor::new(asset))
        .ok()
    // StaticAsset::get(&filename.clone()).map_or_else(
    //     || Err(response::status::NotFound(format!("Bad path: {:?}", path))),
    //     |d| {
    //         Response::build()
    //             .header(content_type)
    //             .sized_body(Cursor::new(d))
    //             .ok()
    //     }
    // )

    // response::NamedFile::open(&path).map_err(|_| response::status::NotFound(format!("Bad path: {:?}", path)))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, static_files])
        .launch();
}