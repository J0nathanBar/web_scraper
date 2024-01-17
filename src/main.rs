use reqwest::blocking::Client;
use scraper::{Html, Selector};
const MENU_URL: &str = "https://www.npmjs.com/search?ranking=popularity&q=react";
const BASE_URL: &str = "https://www.npmjs.com";
fn main() {
    let client = Client::new();

    let response = client
        .get(MENU_URL)
        .send()
        .expect("failed to fetch menu page");

    let body = response.text().expect("failed to extract html");
    //  println!("le body: {}", &body);
    let document = Html::parse_document(&body);

    let option_selector = Selector::parse(".bea55649").expect("invalid selector");

    const N: usize = 100;
    for (index, option) in document.select(&option_selector).take(N).enumerate() {
        if let Some(link) = option.select(&Selector::parse("a").unwrap()).next() {
            // Extract the URL of the linked page from the href attribute
            let extension = link.value().attr("href").unwrap_or_default();
            let linked_page_url = format!("{}/{}", BASE_URL, extension);

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
                println!(
                    "Data from option {} {}: {}",
                    index + 1,
                    extension,
                    data_text
                );
            }
        }
    }
}

/*
fn imdb_example() {
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("h3.ipc-title__text").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
*/
