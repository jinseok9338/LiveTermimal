use std::io::Error;

use crate::config::config::config::Config;
use crate::utils::api::get_projects;
use crate::utils::api::{get_quotes, get_read_me, get_weather};

pub async fn projects(_args: Vec<&str>, config: &'static Config<'static>) -> Result<String, Error> {
    let projects = get_projects(config).await.unwrap();

    let projects_string = projects.into_iter().map(|project|{
       format!(r#"{name} - <a class="text-light-blue dark:text-dark-blue underline" href="{html_url}" target="_blank">{html_url}</a>"#,name=project.name,html_url=project.html_url)
    }).collect::<Vec<String>>().join("\n");

    Ok(projects_string)
}

pub async fn quote(_args: Vec<&str>) -> Result<String, Error> {
    let response = get_quotes().await.unwrap();
    Ok(response.quote)
}

pub async fn read_me(_args: Vec<&str>, config: &'static Config<'static>) -> Result<String, Error> {
    let response = get_read_me(config).await.unwrap();
    Ok(response)
}

pub async fn weather(args: Vec<&str>) -> Result<String, Error> {
    let city = args[1..].join(" ");
    if city.len() == 0 as usize {
        return Ok("Usage: weather [city]. Example: weather casablanca".to_owned());
    }
    let response = get_weather(city).await.unwrap();
    Ok(response)
}
