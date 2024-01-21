use reqwest::Client;
use scraper::{Html, Selector};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio;
const BASE_URL: &str = "https://www.npmjs.com";

#[tokio::main]
async fn main() {
    let page_num: Vec<String> = env::args().collect();
    let start_page_num = page_num[1].parse::<usize>().unwrap();
    let end_page_num = page_num[2].parse::<usize>().unwrap();
    
}

// fn get_npm_page_data(page_link: &str, res_file: Arc<Mutex<File>>) {
//     let mut packages = Vec::new();
//     let client = Client::new();

//     let response = client
//         .get(page_link)
//         .send()
//         .expect("failed to fetch menu page");

//     let body = response.text().expect("failed to extract html");
//     //  println!("le body: {}", &body);
//     let document = Html::parse_document(&body);

//     let option_selector = Selector::parse(".bea55649").expect("invalid selector");

//     for option in document.select(&option_selector) {
//         if let Some(link) = option.select(&Selector::parse("a").unwrap()).next() {
//             // Extract the URL of the linked page from the href attribute
//             let extension = link.value().attr("href").unwrap_or_default();
//             let linked_page_url = format!("{}/{}", BASE_URL, extension);

//             let linked_page_resopnse = client
//                 .get(&linked_page_url)
//                 .send()
//                 .expect("failed to fetch linked page");

//             let linked_page_body = linked_page_resopnse
//                 .text()
//                 .expect("failed to fetch linked body");
//             let linked_page_document = Html::parse_document(&linked_page_body);

//             let data_selector = Selector::parse("._9ba9a726").expect("invalid selector");

//             if let Some(data) = linked_page_document.select(&data_selector).next() {
//                 let data_text = data.text().collect::<Vec<_>>().join(" ");
//                 packages.push(format!("Data from option  {}: {}\n", extension, data_text));
//             }
//         }
//     }
//     let mut file = res_file.lock().unwrap();
//     for package in packages {
//         file.write(package.as_bytes()).unwrap();
//     }
// }
