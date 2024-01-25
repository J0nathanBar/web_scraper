use scraper::Html;

use crate::{git_hub_selector::format_git_api_link, npm_selector::get_git_link};

use self::{fetchers::extract_body, npm_selector::get_package_name};

use super::*;

#[test]
fn check_git_fomat() {
    let result = format_git_api_link("https://github.com/freeCodeCamp/freeCodeCamp");
    assert_eq!(
        result,
        "https://api.github.com/repos/freeCodeCamp/freeCodeCamp"
    );
    let result = format_git_api_link("https://github.com/mui/material-ui");
    assert_eq!(result, "https://api.github.com/repos/mui/material-ui");
}
#[test]
fn check_npm_selector() {
    let doc = get_mui_doc();
    let name = get_package_name(&doc);
    let git_link = get_git_link(&doc);
    assert_eq!(name, Some(String::from("@mui/material")));
    assert_eq!(
        git_link,
        Some(String::from("https://github.com/mui/material-ui"))
    );
}
fn get_mui_doc() -> Html {
    let c = reqwest::blocking::Client::new();
    let body = extract_body(&c, "https://www.npmjs.com/package/@mui/material").unwrap();
    scraper::Html::parse_document(&body)
}
