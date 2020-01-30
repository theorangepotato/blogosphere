#[macro_use]
extern crate lazy_static;

use serde::Deserialize;
use std::fs;
use rss::{Channel, Item};
use std::vec::Vec;
use toml;
use std::collections::HashMap;

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

enum Page {
    Home,
    About,
    Feed {folder: String, index: usize}
}

lazy_static! {
    static ref CONFIG: ConfigFile = toml::from_str(fs::read_to_string("config.toml").unwrap().as_str()).unwrap();
}

fn main() {
    let page = page_to_html(&Page::Feed{folder: "news".to_string(), index: 1});
    print!("{}", page);
}

fn page_to_html(page_requested: &Page) -> String {
    const FEED: &str = "{{FEED}}";
    const NAVIGATION: &str = "{{NAVIGATION}}";
    lazy_static! {
        static ref PAGE_FILE: String = {
            let mut page_file = fs::read_to_string(file_path("templates/page.html")).unwrap();
            let mut navigation = String::from("<ul>");
            for (folder, feeds) in &CONFIG.feeds {
                navigation.push_str(format!("<li>{}</li><ul>", folder).as_str());
                for feed in feeds {
                    navigation.push_str(format!("<li>{}</li>", feed.name).as_str());
                }
                navigation.push_str("</ul>");
            }
            navigation.push_str("</ul>");
            replace(&mut page_file, &NAVIGATION, navigation.as_str());

            page_file
        };
        static ref HOME_FILE: String = fs::read_to_string(file_path("templates/home.html")).unwrap();
        static ref ABOUT_FILE: String = fs::read_to_string(file_path("templates/about.html")).unwrap();
    }

    let mut page = PAGE_FILE.clone();

    match page_requested {
        Page::Home => {
            replace(&mut page, &FEED, &HOME_FILE.as_str());
        }
        Page::About => {
            replace(&mut page, &FEED, &ABOUT_FILE.as_str());
        }
        Page::Feed{folder, index} => {
            let index = *index;
            if !(CONFIG.feeds.contains_key(folder) && CONFIG.feeds[folder].len() > index) {
                panic!("Not a valid feed!");
            }

            let channel = Channel::from_url(&CONFIG.feeds[folder][index].url).expect(format!("Unable to load feed: {}", &CONFIG.feeds[folder][index].name).as_str());
            replace(&mut page, &FEED, feed_to_html(&channel).as_str());
        }
    }

    page
}

fn feed_to_html(channel: &Channel) -> String {
    const TITLE: &str = "{{TITLE}}";
    const FEED: &str = "{{FEED}}";
    lazy_static! {
        static ref FEED_FILE: String = fs::read_to_string(file_path("templates/feed.html")).unwrap();
    }

    let mut feed_file = FEED_FILE.clone();
    replace(&mut feed_file, &TITLE, channel.title());

    let mut feed = String::new();
    let mut item_odd = true;
    for item in channel.items() {
        feed.push_str(feed_item_to_html(&item, item_odd).as_str());
        item_odd = !item_odd;
    }
    replace(&mut feed_file, &FEED, &feed.as_str());

    feed_file
}

fn feed_item_to_html(item: &Item, item_odd: bool) -> String {
    const CLASS: &str = "{{CLASS}}";
    const LINK: &str = "{{LINK}}";
    const TITLE: &str = "{{TITLE}}";
    const CONTENT: &str = "{{CONTENT}}";
    lazy_static! {
        static ref ITEM_FILE: String = fs::read_to_string(file_path("templates/item.html")).unwrap();
    }

    let mut item_file = ITEM_FILE.clone();
    replace(&mut item_file, &CLASS, if item_odd {"item1"} else {"item2"});
    replace(&mut item_file, &LINK, item.link().unwrap());
    replace(&mut item_file, &TITLE, item.title().unwrap());
    replace(&mut item_file, &CONTENT, item.description().unwrap());

    item_file
}

fn replace(source: &mut String, pattern: &str, replacement: &str) {
    let start = source.find(&pattern).unwrap();
    source.replace_range(start..(start + pattern.len()), replacement);
}

#[inline]
fn file_path (relative_path: & str) -> String {
    format!("{}{}", &CONFIG.root, relative_path)
}
