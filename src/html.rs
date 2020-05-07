use super::CONFIG;
use crate::util::file_path;
use rss::{Channel, Item};
use std::fs;

pub enum Page {
    Home,
    About,
    Feed { folder: String, index: usize },
}

const TITLE: &str = "{{TITLE}}";
const FEED: &str = "{{FEED}}";
const NAVIGATION: &str = "{{NAVIGATION}}";
const CLASS: &str = "{{CLASS}}";
const LINK: &str = "{{LINK}}";
const CONTENT: &str = "{{CONTENT}}";
lazy_static! {
    static ref PAGE_FILE: String = {
        let mut page_file = fs::read_to_string(file_path("templates/page.html")).unwrap();
        let mut navigation = String::from("<ul>");
        for (folder, feeds) in &CONFIG.feeds {
            navigation.push_str(format!("<li>{}</li><ul>", folder).as_str());
            for feed_index in 0..feeds.len() {
                navigation.push_str(
                    format!(
                        "<li><a href=\"/{}/{}/\">{}</a></li>",
                        folder, feed_index, feeds[feed_index].name
                    )
                    .as_str(),
                );
            }
            navigation.push_str("</ul>");
        }
        navigation.push_str("</ul>");
        replace(&mut page_file, &NAVIGATION, navigation.as_str());

        page_file
    };
    static ref HOME_FILE: String = fs::read_to_string(file_path("templates/home.html")).unwrap();
    static ref ABOUT_FILE: String = fs::read_to_string(file_path("templates/about.html")).unwrap();
    static ref FEED_FILE: String = fs::read_to_string(file_path("templates/feed.html")).unwrap();
    static ref ITEM_FILE: String = fs::read_to_string(file_path("templates/item.html")).unwrap();
}

pub fn page_to_html(page_requested: &Page) -> String {
    let mut page = PAGE_FILE.clone();

    match page_requested {
        Page::Home => {
            replace(&mut page, &FEED, &HOME_FILE.as_str());
        }
        Page::About => {
            replace(&mut page, &FEED, &ABOUT_FILE.as_str());
        }
        Page::Feed { folder, index } => {
            let index = *index;
            if !(CONFIG.feeds.contains_key(folder) && CONFIG.feeds[folder].len() > index) {
                panic!("Not a valid feed!");
            }

            let feed = &CONFIG.feeds[folder][index];
            let channel = Channel::from_url(&feed.url)
                .expect(format!("Unable to load feed: {}", &feed.name).as_str());
            replace(
                &mut page,
                &FEED,
                feed_to_html(channel.title(), channel.items()).as_str(),
            );
        }
    }

    page
}

fn feed_to_html(title: &str, items: &[Item]) -> String {
    let mut feed_file = FEED_FILE.clone();
    replace(&mut feed_file, &TITLE, title);

    let mut feed = String::new();
    let mut item_odd = true;
    for item in items {
        feed.push_str(feed_item_to_html(&item, item_odd).as_str());
        item_odd = !item_odd;
    }
    replace(&mut feed_file, &FEED, &feed.as_str());

    feed_file
}

fn feed_item_to_html(item: &Item, item_odd: bool) -> String {
    let mut item_file = ITEM_FILE.clone();
    replace(
        &mut item_file,
        &CLASS,
        if item_odd { "item1" } else { "item2" },
    );
    replace(&mut item_file, &LINK, item.link().unwrap());
    replace(&mut item_file, &TITLE, item.title().unwrap());
    replace(&mut item_file, &CONTENT, item.description().unwrap());

    item_file
}

fn replace(source: &mut String, pattern: &str, replacement: &str) {
    let start = source.find(&pattern).unwrap();
    source.replace_range(start..(start + pattern.len()), replacement);
}
