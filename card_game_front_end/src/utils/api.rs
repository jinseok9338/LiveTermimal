use std::fs;

use crate::config::config::config::Config;
use reqwest::{Error, Response};
use serde::Deserialize;

pub async fn get_projects() -> Result<Response, Error> {
    let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");

    let request_url = format!(
        "https://api.github.com/users/{repo}/repos",
        repo = config.social.github
    );
    let response = reqwest::get(&request_url).await?;

    Ok(response)
}

pub async fn get_read_me() -> Result<Response, Error> {
    let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");

    let response = reqwest::get(&config.readme_url).await?;

    Ok(response)
}

pub async fn get_weather(city: String) -> Result<Response, Error> {
    let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");
    let response = reqwest::get(format!("https://wttr.in/${city}?ATm", city = &city)).await?;
    Ok(response)
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
struct QuoteJson {
    _id: String,
    tags: Vec<String>,
    content: String,
    author: String,
    authorSlug: String,
    length: u32,
    dateAdded: String,
    dateModified: String,
}

struct ReturnQuote {
    quote: String,
}

pub async fn get_quotes() -> Result<ReturnQuote, Error> {
    let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");
    let response = reqwest::get("https://api.quotable.io/random").await?;

    let quote_json = response.json::<QuoteJson>().await?;

    Ok(ReturnQuote {
        quote: format!(
            "{content}-{author}",
            content = quote_json.content,
            author = quote_json.author
        ),
    })
}
