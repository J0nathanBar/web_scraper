use scraper::{Html, Selector};

use crate::git_hub_selector::GIT_HUB;

pub struct PackageData {
    pub name: String,
    pub git_link: String,
    pub downloads: usize,
}
pub enum NpmError {
    NoDownloads,
    NotEnoughDownloads,
    NoName,
    NoGitHubRepo,
}
impl PackageData {
    pub fn new_if_downloads(
        download_limit: usize,
        document: &Html,
    ) -> Result<PackageData, NpmError> {
        let downloads = get_package_downloads(&document).ok_or(NpmError::NoDownloads)?;
        if downloads < download_limit {
            return Err(NpmError::NotEnoughDownloads);
        }

        Ok(PackageData {
            downloads,
            name: get_package_name(&document).ok_or(NpmError::NoName)?,
            git_link: get_git_link(&document).ok_or(NpmError::NoGitHubRepo)?,
        })
    }
}

pub fn get_package_name(document: &Html) -> Option<String> {
    let name_selector = Selector::parse("._50685029").unwrap();
    Some(
        document
            .select(&name_selector)
            .next()?
            .text()
            .collect::<Vec<_>>()
            .join(""),
    )
}

pub fn get_git_link(document: &Html) -> Option<String> {
    let git_selector =
        Selector::parse("a[aria-labelledby=\"repository repository-link\"]").unwrap();
    let git_tag = document.select(&git_selector).next()?;
    let link = String::from(git_tag.attr("href")?);
    if link.contains(GIT_HUB) {
        return Some(link);
    }
    None
}

pub fn get_package_downloads(document: &Html) -> Option<usize> {
    let downloads_selector = Selector::parse("._9ba9a726").expect("invalid selector");
    let data = document.select(&downloads_selector).next()?;
    let data_text = data.text().collect::<Vec<_>>().join(" ");
    Some(data_text.replace(",", "").parse::<usize>().unwrap())
}
