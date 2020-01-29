use rss::{Channel, Item};

fn main() {
    let channel = Channel::from_url("https://xkcd.com/rss.xml").unwrap();
    let mut page = String::from("<html>\n<head>\n<link rel=\"stylesheet\" href=\"style.css\"/>\n</head>\n<body>\n");
    page.push_str(channel_to_html(&channel).as_str());
    page.push_str("</body>\n</html>\n");

    print!("{}", page);
}

fn channel_to_html(channel: &Channel) -> String {
    let mut feed = format!("<h2>{}</h2>\n<div id=\"feed\">\n", channel.title());
    let mut item_odd = true;
    for item in channel.items() {
        feed.push_str(print_item(&item, item_odd).as_str());
        item_odd = !item_odd;
    }
    feed.push_str("</div>");

    feed
}

fn print_item(item: &Item, item_odd: bool) -> String {
        format!("<div class=\"{}\">\n\
                <details>\n\
                <summary><a href=\"{}\">{}</a></summary>\n\
                <div class=\"itemContainer\">\n\
                {}\n\
                </div>\n\
                </details>\n\
                </div>\n",
                 if item_odd {"item1"} else {"item2"},
                 item.link().unwrap(),
                 item.title().unwrap(),
                 item.description().unwrap())
}
