use std::{error::Error, fmt::Display};
const ERROR_LENGTH: usize = 20 * 1024;

#[derive(Debug, Clone)]
pub struct ExtractionError {
    cause: String,
    url: String,
}

impl ExtractionError {
    pub fn new(url: &str, cause: &str) -> ExtractionError {
        ExtractionError {
            url: String::from(url),
            cause: String::from(cause),
        }
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}
impl Display for ExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Extraction Error: {} at {}", self.cause, self.url)
    }
}
impl Error for ExtractionError {}

pub fn extract_body(
    client: &reqwest::blocking::Client,
    page_link: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.get(page_link).send()?;
    let body = response.text()?;
    if body.len() < ERROR_LENGTH {
        return Err(Box::new(ExtractionError::new(
            page_link,
            "You are being rate limited!",
        )));
    }
    Ok(body)
}
pub async fn extract_body_async(
    client: &reqwest::Client,
    page_link: &str,
) -> Result<String, reqwest::Error> {
    let response = client.get(page_link).send().await?;
    let body = response.text().await?;
    Ok(body)
}
