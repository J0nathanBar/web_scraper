use reqwest::Client;
use serde_json::Value;

pub const GIT_HUB: &str = "github.com";
const GIT_API: &str = "api.github.com/repos";
pub struct RepoData {
    last_commit: String,
    star_count: u64,
    forks_count: u64,
}
impl RepoData {
    pub fn new(last_commit: &str, star_count: u64, forks_count: u64) -> RepoData {
        RepoData {
            last_commit: String::from(last_commit),
            star_count,
            forks_count,
        }
    }
    pub fn last_commit(&self) -> &str {
        &self.last_commit
    }
    pub fn star_count(&self) -> u64 {
        self.star_count
    }
    pub fn forks_count(&self) -> u64 {
        self.forks_count
    }
}

pub fn get_data(response: &str) -> Option<RepoData> {
    let json: Value = serde_json::from_str(&response).unwrap();
    Some(RepoData::new(
        json["updated_at"].as_str()?,
        json["stargazers_count"].as_u64()?,
        json["forks_count"].as_u64()?,
    ))
}

pub fn format_git_api_link(url: &str) -> String {
    let splices: Vec<&str> = url.split(GIT_HUB).collect();
    format!("{}{}{}", splices[0], GIT_API, splices[1])
}

pub fn build_client() -> Client {
    Client::builder().user_agent("scrooper").build().unwrap()
}
