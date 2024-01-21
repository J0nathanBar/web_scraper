use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::{env, thread, time::Duration};

//const BASE_URL: &str = "https://www.npmjs.com";

fn main() {
    let page_num: Vec<String> = env::args().collect();
    let start_page_num = page_num[1].parse::<usize>().unwrap();
    let end_page_num = page_num[2].parse::<usize>().unwrap();
    let client = Client::new();

    for i in start_page_num..end_page_num {
        let link = format!(
            "https://www.npmjs.com/search?ranking=popularity&q=react&page={}&perPage=20",
            i
        );

        get_npm_page_data(&link, &client);
    }

    println!("done!");
}

fn get_npm_page_data(page_link: &str, client: &Client) {
    let response = client
        .get(page_link)
        .send()
        .expect("failed to fetch menu page");

    let body = response.text().expect("failed to extract html");
    //  println!("le body: {}", &body);
    let document = Html::parse_document(&body);
    loop {
        let mut counter = 0;
        let option_selector = Selector::parse(".bea55649").expect("invalid selector");

        let options: Vec<_> = document.select(&option_selector).collect();

        if options.len() != 20 {
            counter += 1;
            thread::sleep(Duration::from_millis(5));
            if counter > 3 {
                println!("giving up on this unfortunatly");
                break;
            }
        } else {
            if counter > 0 {
                println!("cool reset helps i guess");
            }
            break;
        }
    }
}
