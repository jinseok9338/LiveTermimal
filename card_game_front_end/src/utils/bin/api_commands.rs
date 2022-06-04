use reqwest::Error;

use crate::utils::api::{get_quotes, get_read_me, get_weather};
use crate::utils::{api::get_projects, api_types::Projects};

// // // List of commands that require API calls

// import { getProjects } from '../api';
// import { getQuote } from '../api';
// import { getReadme } from '../api';
// import { getWeather } from '../api';

// export const projects = async (args: string[]): Promise<string> => {
//   const projects = await getProjects();
//   return projects
//     .map(
//       (repo) =>
//         `${repo.name} - <a class="text-light-blue dark:text-dark-blue underline" href="${repo.html_url}" target="_blank">${repo.html_url}</a>`,
//     )
//     .join('\n');
// };

// export const quote = async (args: string[]): Promise<string> => {
//   const data = await getQuote();
//   return data.quote;
// };

// export const readme = async (args: string[]): Promise<string> => {
//   const readme = await getReadme();
//   return `Opening GitHub README...\n
//   ${readme}`;
// };

// export const weather = async (args: string[]): Promise<string> => {
//   const city = args.join('+');
//   if (!city) {
//     return 'Usage: weather [city]. Example: weather casablanca';
//   }
//   const weather = await getWeather(city);
//   return weather;
// };

pub async fn projects(args: Vec<String>) -> Result<String, Error> {
    let response = get_projects().await.unwrap();
    let projects = response.json::<Vec<Projects>>().await?;

    let projects_string = projects.into_iter().map(|project|{
       format!(r#"{name} - <a class="text-light-blue dark:text-dark-blue underline" href="{html_url}" target="_blank">{html_url}</a>"#,name=project.name,html_url=project.html_url)
    }).collect::<Vec<String>>().join("\n");

    Ok(projects_string)
}

pub async fn quote(args: Vec<String>) -> Result<String, Error> {
    let response = get_quotes().await.unwrap();
    Ok(response.quote)
}

pub async fn read_me(args: Vec<String>) -> Result<String, Error> {
    let response = get_read_me().await.unwrap();
    Ok(response)
}

pub async fn weather(args: Vec<String>) -> Result<String, Error> {
    let city = args.join("+");
    if city.len() == 0 {
        panic!("Usage: weather [city]. Example: weather casablanca")
    }
    let response = get_weather(city).await.unwrap();
    Ok(response)
}
