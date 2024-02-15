use std::{fs, path::Path};

use soup::NodeExt;
use soup::Soup;

fn main() {
    println!("Hello, world!");

    let main_html_link = Path::new("src/main.html");

    let content: String = fs::read_to_string(main_html_link).unwrap();
    let soup = Soup::new(&content);
    // hyperlinks -> get all hyperlinks from the content

    // <a href="https://www.google.com">Google</a>
    let all_tags_a_href = soup.tag("a").find_all();

    all_tags_a_href.enumerate().for_each(|(_, tag)| {
        let href = tag.get("href");
        match href {
            Some(href) => {
                if href.starts_with("http") || href.starts_with('_') || href.starts_with('#') {
                    return;
                }

                let final_link = main_html_link.parent().unwrap().join(href.clone());

                println!("{:?}", final_link);
            }
            None => println!("No href attribute found"),
        }
    });

    let results = soup.text();

    // println!("{:?}", results);
}
