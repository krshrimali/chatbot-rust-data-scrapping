// use std::{fs, path::Path};

use std::collections::HashSet;
use std::path::Path;

use soup::NodeExt;
use soup::Soup;

use indicatif::ProgressBar;
mod prompt_generator;
use prompt_generator::{Dataset, PromptGenerator};

pub fn extract_all_hyperlinks(
    raw_data: Result<String, std::io::Error>,
    all_links: &mut Vec<String>,
    all_text: &mut Vec<String>,
) -> Result<(), std::io::Error> {
    match raw_data {
        Ok(content) => {
            let soup = Soup::new(&content);
            let text = soup.text();
            all_text.push(text);
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
    let mut all_text: Vec<String> = Vec::new();

    let if_succeeded = extract_all_hyperlinks(raw_data, &mut all_links, &mut all_text);
    let mut nested_links: Vec<String> = Vec::new();
    if let Err(e) = if_succeeded {
        log::error!("Error: {:?}", e);
        return;
    }

    let mut visited_set: HashSet<String> = HashSet::new();
    let mut total_visited_alr_and_skipped: u32 = 0;
    let mut total_visited_and_successful: u32 = 0;
    let mut total_failed: u32 = 0;

    // progress bar in rust
    let pb = ProgressBar::new(all_links.len() as u64);
    for link in all_links.iter() {
        if visited_set.contains(link) {
            log::info!("Skipping link because it's alr visited: {:?}", link);
            total_visited_alr_and_skipped += 1;
            pb.inc(1);
            continue;
        }

        let modified_link = Path::new(&main_html_link).parent().unwrap().join(link);
        let content = fetch_raw_html(modified_link.display().to_string());
        let success = extract_all_hyperlinks(content, &mut nested_links, &mut all_text);
        visited_set.insert(link.clone());
        if success.is_ok() {
            log::info!("Success for link: {:?}", link);
            total_visited_and_successful += 1;
            log::debug!("Nested links: {:?}", nested_links);
        } else {
            log::info!("Failed for link: {:?}", link);
            total_failed += 1;
        }
        pb.inc(1);
    }
    log::info!("All links: {:?}", all_links);

    println!("Statistics: ");
    println!(
        "Total visited and successful: {:?}",
        total_visited_and_successful
    );
    println!(
        "Total visited and skipped: {:?}",
        total_visited_alr_and_skipped
    );
    println!("Total failed: {:?}", total_failed);
    println!("All text fetched: {:?}", all_text.len());

    // write all_text to a single file
    let text_pb = ProgressBar::new(all_text.len() as u64);

    std::fs::create_dir_all("output").expect("Failed to create the directory");

    let mut p_generator = PromptGenerator::default();
    p_generator = p_generator.init();

    // <row -> prompt, answer>, <....>
    let mut whole_dataset = Dataset::default();

    for (idx, text) in all_text.iter().enumerate() {
        let file_name = format!("output/output_{}.txt", idx);
        std::fs::write(file_name, text).expect("Failed to write the file");

        let output_text = p_generator.generate_output(text);
        let dataset = p_generator.process_output_text_into_dataset(output_text);
        // dataset format is: <prompt, answer>
        whole_dataset.extend(dataset);
        text_pb.inc(1);
    }
}
