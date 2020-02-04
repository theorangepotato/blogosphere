#[macro_use]
extern crate lazy_static;

mod html;

use crate::html::{Page, page_to_html};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::vec::Vec;
use toml;

#[derive(Deserialize)]
pub struct Feed {
    name: String,
    url: String
}

#[derive(Deserialize)]
pub struct ConfigFile {
    root: String,
    feeds: HashMap<String, Vec<Feed>>
}

lazy_static! {
    pub static ref CONFIG: ConfigFile = toml::from_str(fs::read_to_string("config.toml").unwrap().as_str()).unwrap();
}

fn main() {
    let page = page_to_html(&Page::Feed{folder: "news".to_string(), index: 1});
    print!("{}", page);
}
