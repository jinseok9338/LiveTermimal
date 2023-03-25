use std::io::Error;

use super::api_types::{Projects, QuoteJson};
use crate::config::config::config::Config;
use crate::utils::api_types::ReturnQuote;
use lazy_static::lazy_static;

use serde_wasm_bindgen;


use regex::Regex;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub async fn get_projects(config: Config) -> Result<Vec<Projects>, JsValue> {
    let request_url = format!(
        "https://api.github.com/users/{}/repos",
        config.social.github
    );
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = Request::new_with_str_and_init(&request_url, &opts)?;

    let window = web_sys::window().unwrap();
    let fetch_with_request = window.fetch_with_request(&request);
    let resp_value = JsFuture::from(fetch_with_request).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json_promise = resp.json()?;
    let json = JsFuture::from(json_promise).await?;
    let projects: Vec<Projects> = serde_wasm_bindgen::from_value(json)?;
    Ok(projects)
}


pub async fn get_readme(config: &Config) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = Request::new_with_str_and_init(&config.readme_url, &opts)?;
    
    let window = web_sys::window().unwrap();
    let fetch_with_request = window.fetch_with_request(&request);
    let resp_value = JsFuture::from(fetch_with_request).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let text_promise = resp.text()?;
    let text = JsFuture::from(text_promise).await?;
    let text_str: String = text.as_string().unwrap();
    Ok(text_str)
}

fn some_helper_function(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<a href[\s\S]*</script>").unwrap();
    }
    RE.replace_all(text, "").to_string()
}

pub async fn get_weather(city: String) -> Result<String, JsValue> {
    let url = format!("https://wttr.in/{}", city);
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let fetch_with_request = window.fetch_with_request(&request);
    let resp_value = JsFuture::from(fetch_with_request).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let text_promise = resp.text().expect("Failed to read response text");
    let text = JsFuture::from(text_promise).await?;
    let text_str: String = text.as_string().unwrap();
    let trimmed_string = some_helper_function(&text_str);

    Ok(trimmed_string)
}

pub async fn get_quotes() -> Result<ReturnQuote, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = Request::new_with_str_and_init("https://api.quotable.io/random", &opts)?;

    let window = web_sys::window().unwrap();
       let fetch_with_request = window.fetch_with_request(&request);
    let resp_value = JsFuture::from(fetch_with_request).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json_promise = resp.json()?;
    let json = JsFuture::from(json_promise).await?;
    let quote: QuoteJson = serde_wasm_bindgen::from_value(json)?;
  
    Ok(ReturnQuote {
        quote: format!(
            "{content} - {author}",
            content = quote.content,
            author = quote.author
        ),
    })
}
