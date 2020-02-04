#[macro_use]
extern crate lazy_static;

mod html;
mod util;

use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::vec::Vec;
use toml;
use util::file_path;

#[derive(Deserialize)]
struct Feed {
    name: String,
    url: String
}

#[derive(Deserialize)]
struct ConfigFile {
    root: String,
    feeds: HashMap<String, Vec<Feed>>
}

lazy_static! {
    static ref CONFIG: ConfigFile = toml::from_str(fs::read_to_string("config.toml").unwrap().as_str()).unwrap();
}

#[get("/")]
async fn return_home() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
       .content_type("text/html; charset=utf-8")
       .body(html::page_to_html(&html::Page::Home)))
}

#[get("/about.html")]
async fn return_about() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
       .content_type("text/html; charset=utf-8")
       .body(html::page_to_html(&html::Page::About)))
}

#[get("/style.css")]
async fn return_css() -> impl Responder {
    fs::read_to_string(file_path("style.css")).unwrap()
}

#[get("/{folder}/{index}/")]
async fn return_feed(info: web::Path<(String, usize)>) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
       .content_type("text/html; charset=utf-8")
       .body(html::page_to_html(&html::Page::Feed{folder : info.0.clone(), index : info.1})))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
                    .service(return_home)
                    .service(return_about)
                    .service(return_css)
                    .service(return_feed))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
