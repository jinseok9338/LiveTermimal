use reqwest::Error;

use crate::utils::api::{get_quotes, get_read_me, get_weather};
use crate::utils::{api::get_projects, api_types::Projects};

#[allow(dead_code)]
pub async fn projects(_args: Vec<String>) -> Result<String, Error> {
    let response = get_projects().await.unwrap();
    let projects = response.json::<Vec<Projects>>().await?;

    let projects_string = projects.into_iter().map(|project|{
       format!(r#"{name} - <a class="text-light-blue dark:text-dark-blue underline" href="{html_url}" target="_blank">{html_url}</a>"#,name=project.name,html_url=project.html_url)
    }).collect::<Vec<String>>().join("\n");

    Ok(projects_string)
}
#[allow(dead_code)]
pub async fn quote(_args: Vec<String>) -> Result<String, Error> {
    let response = get_quotes().await.unwrap();
    Ok(response.quote)
}
#[allow(dead_code)]
pub async fn read_me(_args: Vec<String>) -> Result<String, Error> {
    let response = get_read_me().await.unwrap();
    Ok(response)
}
#[allow(dead_code)]
pub async fn weather(args: Vec<String>) -> Result<String, Error> {
    let city = args.join("+");
    if city.len() == 0 {
        panic!("Usage: weather [city]. Example: weather casablanca")
    }
    let response = get_weather(city).await.unwrap();
    Ok(response)
}
