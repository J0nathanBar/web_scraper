use futures::{stream, StreamExt};
use reqwest::Client;
use scraper::{Html, Selector};
use std::{env, fs, io::Write};
use tokio;
const BASE_URL: &str = "https://www.npmjs.com";
#[tokio::main]
async fn main() {
    let client = Client::new();
    let page_num: Vec<String> = env::args().collect();
    let start_page_num = page_num[1].parse::<usize>().unwrap();
    let end_page_num = page_num[2].parse::<usize>().unwrap();
    let mut links = Vec::new();
    for i in start_page_num..end_page_num {
        links.push(format!(
            "https://www.npmjs.com/search?ranking=popularity&q=react&page={}&perPage=20",
            i
        ));
    }
    let bodies = stream::iter(links)
        .map(|url| {
            let client = client.clone();
            async move {
                let resp = client.get(&url).send().await?;
                let body = resp.text().await?;

                get_options(client, body).await
            }
        })
        .buffer_unordered(end_page_num - start_page_num);

    bodies
        .for_each(|b| async {
            match b {
                Ok(b) if b.0.len() != 20 => {
                  //  println!("fuckk we gott {}", b.0.len());
                //    fs::write(format!("test{}.html", b.0.len()), b.1).unwrap();
                }

                Ok(b) => {
                    let mut file = fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open("res.txt")
                        .unwrap();
                    for package in b.0 {
                        file.write(package.as_bytes()).unwrap();
                    }
                }
                Err(e) => eprintln!("got an error : {}", e),
            }
        })
        .await;
}

async fn get_options(
    client: Client,

    og_body: String,
) -> Result<(Vec<String>, String), reqwest::Error> {
    let mut packages = Vec::new();
    let document = Html::parse_document(&og_body);
    let option_selector = Selector::parse(".bea55649").unwrap();
    let ves: Vec<_> = document.select(&option_selector).collect();
    println!("ves is {}", ves.len());
    // for option in document.select(&option_selector) {
    //     let link = option
    //         .select(&Selector::parse("a").unwrap())
    //         .next()
    //         .unwrap();
    //     // Extract the URL of the linked page from the href attribute
    //     let extension = link.value().attr("href").unwrap_or_default();
    //     let linked_page_url = format!("{}/{}", BASE_URL, extension);

    //     let linked_page_resopnse = client.get(&linked_page_url).send().await?;

    //     let linked_page_body = linked_page_resopnse.text().await?;
    //     let linked_page_document = Html::parse_document(&linked_page_body);

    //     let data_selector = Selector::parse("._9ba9a726").unwrap();

    //     if let Some(data) = linked_page_document.select(&data_selector).next() {
    //         let data_text = data.text().collect::<Vec<_>>().join(" ");
    //         packages.push(format!("Data from option  {}: {}\n", extension, data_text));
    //     }
    // }

    Ok((packages, og_body))
}
