use reqwest::{
    blocking::{Client, Response},
    Body, Request,
};
use scraper::{ElementRef, Html, Selector};
use std::{env, fs, io::Write, path, thread, time::Duration};

const BASE_URL: &str = "https://www.npmjs.com";
const ERROR_LENGTH: usize = 20 * 1024;

fn main() {
    let page_num: Vec<String> = env::args().collect();
    let start_page_num = page_num[1].parse::<usize>().unwrap();
    let end_page_num = page_num[2].parse::<usize>().unwrap();
    iterate_pages(start_page_num, end_page_num);
    //iterate_packages();
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

fn iterate_packages() {
    let paths: Vec<_> = fs::read_dir("htmls/packages").unwrap().collect();
    println!("{}", paths.len());
}
