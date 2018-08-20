extern crate actix_web;
extern crate env_logger;
extern crate comrak;
extern crate chrono;

#[macro_use]
extern crate askama;

#[macro_use]
extern crate rust_embed;
extern crate mime_guess;

mod note;

use actix_web::http::Method;
use actix_web::middleware::Logger;
use actix_web::{server, App, HttpRequest, HttpResponse, Result};
use askama::Template;
use mime_guess::guess_mime_type;
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
struct IndexTemplate<'a> {
    debug: bool,
    name: &'a str,
    markdown: String,
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAsset;

fn handle_static_file(path: &str) -> HttpResponse {
    match StaticAsset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(guess_mime_type(path).as_ref())
            .body(content),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let s = IndexTemplate {
        debug: cfg!(debug_assertions),
        name: "Luke",
        markdown: markdown_to_html("This **is** a test\n\n + [ ] this is a checkbox", &ComrakOptions { ext_tasklist: true, ..ComrakOptions::default()}),
    }.render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn static_files(req: HttpRequest) -> HttpResponse {
    let path = &req.path()["/static/".len()..]; // trim the preceding `/sstatic/` in path
    handle_static_file(path)
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    server::new(|| {
        App::new()
            .middleware(Logger::default())
            .route("/", Method::GET, index)
            .route("/static/{_:.*}", Method::GET, static_files)
    }).bind("127.0.0.1:8000")
    .unwrap()
    .run();
}
