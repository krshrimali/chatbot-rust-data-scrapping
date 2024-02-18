// use std::{fs, path::Path};

use std::collections::HashSet;
use std::path::Path;

use soup::NodeExt;
use soup::Soup;
// use log::{debug, info, error, warn};

pub fn extract_all_hyperlinks(
    raw_data: Result<String, std::io::Error>,
    all_links: &mut Vec<String>,
) -> Result<(), std::io::Error> {
    match raw_data {
        Ok(content) => {
            let soup = Soup::new(&content);
            let all_tags_a_href = soup.tag("a").find_all();

            all_tags_a_href.enumerate().for_each(|(_, tag)| {
                let href = tag.get("href");
                match href {
                    Some(href) => {
                        // TODO: Have a better way to organise these prefixes
                        if href.starts_with("http")
                            || href.starts_with('_')
                            || href.starts_with('#')
                            || href.starts_with('.')
                        {
                            log::info!("Skipping: {:?}", href);
                            return;
                        }

                        // let final_link = main_html_link.parent().unwrap().join(href.clone());
                        // log::info!("Final link: {:?}", final_link);
                        all_links.push(href);
                    }
                    None => log::warn!("No href attribute found"),
                }
            });
            Ok(())
        }
        Err(e) => {
            log::error!("Error reading file: {:?}", e,);
            Err(e)
        }
    }
}

pub fn fetch_raw_html(main_html_link: String) -> Result<String, std::io::Error> {
    let output = reqwest::blocking::get(main_html_link).expect("Failed to fetch the URL");
    // let output_document = Document::from(output.unwrap().text().unwrap().as_str());
    let output_text = output.text().expect("Failed to read the response text");
    Ok(output_text.to_string())
}

fn main() {
    let main_html_link: String = "https://pytorch.org/docs/stable/index.html".to_string();
    let raw_data: Result<String, std::io::Error> = fetch_raw_html(main_html_link.clone());

    env_logger::init();

    let mut all_links: Vec<String> = Vec::new();
    let if_succeeded = extract_all_hyperlinks(raw_data, &mut all_links);
    let mut nested_links: Vec<String> = Vec::new();
    if let Err(e) = if_succeeded {
        log::error!("Error: {:?}", e);
        return;
    }

    let mut visited_set: HashSet<String> = HashSet::new();
    for link in all_links.iter() {
        if visited_set.contains(link) {
            log::info!("Skipping link because it's alr visited: {:?}", link);
            continue;
        }
        let modified_link = Path::new(&main_html_link).parent().unwrap().join(link);
        let content = fetch_raw_html(modified_link.display().to_string());
        let success = extract_all_hyperlinks(content, &mut nested_links);
        visited_set.insert(link.clone());
        if success.is_ok() {
            log::info!("Success for link: {:?}", link);
            log::debug!("Nested links: {:?}", nested_links);
        }
    }
    log::info!("All links: {:?}", all_links);
}
