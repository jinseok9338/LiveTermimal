use std::io::Error;

use super::api_types::{Projects, QuoteJson};
use crate::config::config::config::Config;
use crate::utils::api_types::ReturnQuote;
use lazy_static::lazy_static;

use gloo_console::log;
use regex::Regex;
use reqwasm::http::Request;

pub async fn get_projects(config: Config) -> Result<Vec<Projects>, Error> {
    let request_url = format!(
        "https://api.github.com/users/{repo}/repos",
        repo = config.social.github
    );

    let fetched_json: Vec<Projects> = Request::get(&request_url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(fetched_json)
}

pub async fn get_read_me(config: Config) -> Result<String, Error> {
    let fetched_string: String = Request::get(&config.readme_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(fetched_string)
}

fn some_helper_function(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<a href[\s\S]*</script>").unwrap();
    }
    RE.replace_all(text, "").to_string()
}

pub async fn get_weather(city: String) -> Result<String, Error> {
    let fetched_string: String = Request::get(&format!("https://wttr.in/{city}", city = &city))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let trimmed_string = some_helper_function(&fetched_string);

    Ok(trimmed_string)
}

pub async fn get_quotes() -> Result<ReturnQuote, Error> {
    let fetched_json: QuoteJson = Request::get("https://api.quotable.io/random")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(ReturnQuote {
        quote: format!(
            "{content} - {author}",
            content = fetched_json.content,
            author = fetched_json.author
        ),
    })
}
