use std::error::Error;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};

const COOKIE_VALUE: &str = "";

pub async fn fetch_data(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(COOKIE_VALUE)?);

    let response = client.get(url)
        .headers(headers)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body_text = response.text().await?;
        println!("Unexpected response status: {}, body: {}", status, body_text);
        return Err(format!("Unexpected response status: {}", status).into());
    }

    let body_text = response.text().await?;

    let lines: Vec<String> = body_text.lines().map(|line| line.to_string()).collect();

    Ok(lines)
}