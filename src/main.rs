use std::fs;
use rss::{Channel, Item};
use std::vec::Vec;

fn main() {
    let mut channels = Vec::new();
    channels.push(Channel::from_url("https://xkcd.com/rss.xml").unwrap());
    let page = construct_page(&channels);
    print!("{}", page);
}

fn construct_page(channels: &Vec<Channel>) -> String {
    const PAGE: &str = "{{PAGE}}";

    let mut page = fs::read_to_string("templates/page.html").unwrap();
    replace(&mut page, &PAGE, channel_to_html(&channels[0]).as_str());

    page
}

fn channel_to_html(channel: &Channel) -> String {
    const TITLE: &str = "{{TITLE}}";
    const FEED: &str = "{{FEED}}";

    let mut feed_file = fs::read_to_string("templates/feed.html").unwrap();
    replace(&mut feed_file, &TITLE, channel.title());

    let mut feed = String::new();
    let mut item_odd = true;
    for item in channel.items() {
        feed.push_str(print_item(&item, item_odd).as_str());
        item_odd = !item_odd;
    }
    replace(&mut feed_file, &FEED, &feed.as_str());

    feed_file
}

fn print_item(item: &Item, item_odd: bool) -> String {
    const CLASS: &str = "{{CLASS}}";
    const LINK: &str = "{{LINK}}";
    const TITLE: &str = "{{TITLE}}";
    const CONTENT: &str = "{{CONTENT}}";

    let mut item_file = fs::read_to_string("templates/item.html").unwrap();
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
