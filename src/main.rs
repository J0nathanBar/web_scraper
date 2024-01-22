use reqwest::blocking::Client;
use scraper::{ElementRef, Html, Selector};
use std::{env, fs, io::Write, thread, time::Duration};

const BASE_URL: &str = "https://www.npmjs.com";

fn main() {
    let page_num: Vec<String> = env::args().collect();
    let start_page_num = page_num[1].parse::<usize>().unwrap();
    let end_page_num = page_num[2].parse::<usize>().unwrap();
    //   let mut packages = Vec::new();

    let client = Client::new();
    let option_selector = Selector::parse(".bea55649").expect("invalid selector");

    for i in start_page_num..end_page_num {
        println!("page {i}/{end_page_num}");
        let link = format!(
            "https://www.npmjs.com/search?ranking=popularity&q=react&page={}&perPage=20",
            i
        );

        // get_npm_page_data(&link, &client, &mut packages, &option_selector);
        save_html_to_pc(&client, &link,i);
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("le_res.txt")
        .unwrap();
    // for package in packages {
    //     file.write(package.as_bytes()).unwrap();
    // }
    println!("done!");
}

fn get_npm_page_data(
    page_link: &str,
    client: &Client,
    packages: &mut Vec<String>,
    option_selector: &Selector,
) {
    //thread::sleep(Duration::from_secs(1));

    let response = client
        .get(page_link)
        .send()
        .expect("failed to fetch menu page");

    let body = response.text().expect("failed to extract html");
    let document = Html::parse_document(&body);

    for option in document.select(&option_selector) {
        get_npm_package(option, client, packages);
    }
}

fn get_npm_package(option: ElementRef<'_>, client: &Client, packages: &mut Vec<String>) {
    if let Some(link) = option.select(&Selector::parse("a").unwrap()).next() {
        // Extract the URL of the linked page from the href attribute
        let extension = link.value().attr("href").unwrap_or_default();
        let linked_page_url = format!("{}/{}", BASE_URL, extension);
        thread::sleep(Duration::from_secs(1));

        let linked_page_resopnse = client
            .get(&linked_page_url)
            .send()
            .expect("failed to fetch linked page");

        let linked_page_body = linked_page_resopnse
            .text()
            .expect("failed to fetch linked body");
        let linked_page_document = Html::parse_document(&linked_page_body);

        let data_selector = Selector::parse("._9ba9a726").expect("invalid selector");

        if let Some(data) = linked_page_document.select(&data_selector).next() {
            let data_text = data.text().collect::<Vec<_>>().join(" ");
            packages.push(format!("Data from option  {}: {}\n", extension, data_text));
        } else {
            println!("fuckkk! in the new selection")
        }
    } else {
        println!("fuckkk! in the og selection")
    }
}

fn save_html_to_pc(client: &Client, page_link: &str,index:usize) {
    let response = client
        .get(page_link)
        .send()
        .expect("failed to fetch menu page");

    let body = response.text().expect("failed to extract html");
    let pathh = format!("htmls/menu_pages/{}.html", index);
    fs::write(pathh, body).unwrap();
    thread::sleep(Duration::from_secs(1));
}
