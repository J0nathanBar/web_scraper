use std::{
    fs,
    sync::{Arc, Mutex},
};

use futures::{stream, StreamExt};
use git_hub_selector::format_git_api_link;
use npm_selector::PackageData;
use scraper::Html;

use crate::{
    fetchers::extract_body_async,
    git_hub_selector::{build_client, get_data},
};

pub mod fetchers;
pub mod git_hub_selector;
pub mod npm_selector;

const DOWNLOAD_LIMIT: usize = 100000;
#[cfg(test)]
mod tests;

pub async fn iterate_packages() {
    let client = build_client();
    let links = get_git_links();
    println!("got them linkies");
    let repos_data = stream::iter(links)
        .map(|link| {
            let client = client.clone();
            async move { (extract_body_async(&client, &link).await, link) }
        })
        .buffer_unordered(6000);
    repos_data
        .for_each(|(response, url)| async move {
            match response {
                Ok(response) => {
                    let git_data = get_data(&response);
                    match git_data {
                        Some(git_data) => {
                            if git_data.star_count() > 1000 {
                                println!("we got a winner! ding ding ding{}", url);
                            }
                            else {
                                println!("welp at least i exist");
                            }
                        }
                        _ => eprintln!("we done goofed. here link: {url}"),
                    }
                }
                Err(e) => {
                    eprintln!("Got an error: {}", e);
                }
            }
        })
        .await;
}

fn get_git_links() -> Vec<String> {
    fs::read_dir("htmls/packages")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            let package_body = String::from_utf8(fs::read(&path).unwrap()).unwrap();
            let document = Html::parse_document(&package_body);
            let link = format_git_api_link(
                &PackageData::new_if_downloads(DOWNLOAD_LIMIT, &document)
                    .ok()?
                    .git_link,
            );
            Some(link)
        })
        .collect()
}
