use std::fs;

use super::api_types::QuoteJson;
use crate::config::config::config::Config;
use crate::utils::api_types::ReturnQuote;
use reqwest::{Error, Response};

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

pub async fn get_read_me() -> Result<String, Error> {
    let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");

    let response = reqwest::get(&config.readme_url).await?;
    let response_string = response.text().await.unwrap();

    Ok(response_string)
}

pub async fn get_weather(city: String) -> Result<String, Error> {
    let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");
    let response = reqwest::get(format!("https://wttr.in/${city}?ATm", city = &city)).await?;
    let response_text = response.text().await.unwrap();
    Ok(response_text)
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
