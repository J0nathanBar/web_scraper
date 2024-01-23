use futures::{stream, StreamExt};
use reqwest::{
    blocking::{Client, Response},
    Body, Error, Request,
};
use scraper::{ElementRef, Html, Selector};
use std::{env, fs, io::Write, path, thread, time::Duration, usize};
use tokio;
const BASE_URL: &str = "https://www.npmjs.com";
const ERROR_LENGTH: usize = 20 * 1024;
const DOWNLOAD_LIMIT: usize = 100000;

#[tokio::main]
async fn main() {
    // let page_num: Vec<String> = env::args().collect();
    // let start_page_num = page_num[1].parse::<usize>().unwrap();
    // let end_page_num = page_num[2].parse::<usize>().unwrap();
    // // iterate_pages(start_page_num, end_page_num);

    iterate_packages().await;
}

fn save_html_to_pc(client: &Client, page_link: &str, index: usize) {
    let response = client
        .get(page_link)
        .send()
        .expect("failed to fetch menu page");

    let body = response.text().expect("failed to extract html");
    let pathh = format!("htmls/menu_pages/{}.html", index);
    fs::write(pathh, body).unwrap();
    thread::sleep(Duration::from_secs(1));
}

fn iterate_pages(start_page_num: usize, end_page_num: usize) {
    let mut packages = Vec::new();
    for i in start_page_num..end_page_num {
        let path = format!("htmls/menu_pages/{i}.html");

        let html_file = String::from_utf8(fs::read(path).unwrap()).unwrap();
        let document = Html::parse_document(&html_file);
        let option_selector = Selector::parse(".bea55649").expect("invalid selector");

        for option in document.select(&option_selector) {
            if let Some(link) = option.select(&Selector::parse("a").unwrap()).next() {
                // Extract the URL of the linked page from the href attribute
                let extension = link.value().attr("href").unwrap_or_default();
                let linked_page_url = format!("{}/{}", BASE_URL, extension);
                packages.push(linked_page_url);
            }
        }
    }
    let client = Client::new();

    for (index, package) in packages.iter().enumerate() {
        println!("{index}/{}", packages.len());
        download_packages(&client, &package);
    }
}

fn download_packages(client: &Client, page_link: &str) {
    let package_name = page_link.split('/').last().unwrap_or("");
    let pathh = format!("htmls/packages/{}.html", package_name);
    if fs::metadata(&pathh).is_ok() {
        println!("uwu i exist!");
        return;
    }

    let body: String;
    let mut counter = 0;
    loop {
        match extract_body(client, page_link) {
            Ok(b) if b.len() > ERROR_LENGTH => {
                body = b;
                break;
            }
            Ok(_) => {
                counter += 1;
                thread::sleep(Duration::from_secs(3));
                if counter > 5 {
                    println!("deep shit");
                    thread::sleep(Duration::from_secs(60))
                };
            }
            Err(e) => {
                eprintln!("{}", e);
                counter += 1;
                thread::sleep(Duration::from_secs(60))
            }
        }
    }
    fs::write(pathh, body).unwrap();
}
fn extract_body(client: &Client, page_link: &str) -> Result<String, reqwest::Error> {
    let response = client.get(page_link).send()?;
    let body = response.text()?;
    Ok(body)
}

async fn iterate_packages() {
    let client = reqwest::Client::new();
    let links = get_git_links();
    println!("got them linkies");
    let bodies = stream::iter(links)
        .map(|link| {
            let client = client.clone();
            async move {
                let mut counter = 0;
                loop {
                    let body = extract_body_async(&client, link.as_str()).await;
                    if let Ok(body) = body {
                        if counter > 1 {
                            println!("oh look problem solved after {counter} iterations");
                        }
                        return Ok(body);
                    } else if counter > 5 {
                        return Err(format!("sorry bro i tried "));
                    }
                    counter += 1;
                    thread::sleep(Duration::from_millis(3));
                }
            }
        })
        .buffer_unordered(6000);
    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => println!("Got {} ", b.len()),
                Err(e) => eprintln!("Got an error: {}", e),
            }
        })
        .await;

    // for (index, package) in packages.enumerate() {
    //     let path = package.unwrap().path();
    //     let package_body = String::from_utf8(fs::read(&path).unwrap()).unwrap();
    //     let document = Html::parse_document(&package_body);
    //     // let package_name = get_package_name(&document, &name_selector);
    //     let downloads = extract_package_downloads(&data_selector, &document);
    //     match downloads {
    //         Some(downloads) if downloads > DOWNLOAD_LIMIT => {
    //             if let Some(git) = extract_git_hub_page(&document, &git_selector) {
    //                 match extract_body_async(client.clone(), &git).await {
    //                     Ok(b) if b.len() > ERROR_LENGTH => {
    //                         println!("whoohoo we're at {index}")
    //                     }
    //                     _ => {
    //                         eprintln!("we reached limit at : {index}");
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         _ => {}
    //     }
    // }
}

fn extract_package_downloads(data_selector: &Selector, document: &Html) -> Option<usize> {
    if let Some(data) = document.select(&data_selector).next() {
        let data_text = data.text().collect::<Vec<_>>().join(" ");
        return Some(data_text.replace(",", "").parse::<usize>().unwrap());
    }
    None
}

fn extract_git_hub_page(document: &Html, git_selector: &Selector) -> Option<String> {
    let git_tag = document.select(&git_selector).next()?;
    Some(String::from(git_tag.attr("href")?))
}

fn get_package_name(document: &Html, name_selector: &Selector) -> String {
    document
        .select(&name_selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()
        .join("")
}

async fn extract_body_async(
    client: &reqwest::Client,
    page_link: &str,
) -> Result<String, reqwest::Error> {
    let response = client.get(page_link).send().await?;
    let body = response.text().await?;
    Ok(body)
}

fn get_git_links() -> Vec<String> {
    let data_selector = Selector::parse("._9ba9a726").expect("invalid selector");
    //let name_selector = Selector::parse("._50685029").unwrap();
    let git_selector =
        Selector::parse("a[aria-labelledby=\"repository repository-link\"]").unwrap();
    fs::read_dir("htmls/packages")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            let package_body = String::from_utf8(fs::read(&path).unwrap()).unwrap();
            let document = Html::parse_document(&package_body);
            let downloads = extract_package_downloads(&data_selector, &document);
            if let Some(downloads) = downloads {
                if downloads > DOWNLOAD_LIMIT {
                    if let Some(git_link) = extract_git_hub_page(&document, &git_selector) {
                        return Some(git_link);
                    }
                }
            }
            None
        })
        .collect()
}
