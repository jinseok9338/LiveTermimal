use web_sys::Node;

use crate::config::command_config::config::Config;
use crate::utils::api::get_projects;
use crate::utils::api::{get_quotes, get_read_me, get_weather};

use super::add_element::{add_loading, remove_loading};
use super::execute_command::ShellCommandReturnType;
use super::programs::legacy::LegacyProps;
use super::programs::programs::OutputComponent;

pub async fn projects(
    _args: Vec<&str>,
    config: &'static Config<'static>,
) -> ShellCommandReturnType {
    let projects = get_projects(config).await.unwrap();

    let projects_string = projects.into_iter().map(|project|{
       format!(r#"{name} - <a class="text-light-blue dark:text-dark-blue underline" href="{html_url}" target="_blank">{html_url}</a>"#,name=project.name,html_url=project.html_url)
    }).collect::<Vec<String>>().join("\n");
    let operation = None;
    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: projects_string,
    }));
    Ok((output_component, operation))
}

pub async fn quote(_args: Vec<&str>) -> ShellCommandReturnType {
    let response = get_quotes().await.unwrap();
    let operation = None;
    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: response.quote,
    }));
    Ok((output_component, operation))
}

pub async fn read_me(_args: Vec<&str>, config: &'static Config<'static>) -> ShellCommandReturnType {
    let response = get_read_me(config).await.unwrap();
    let operation = None;
    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: response,
    }));
    Ok((output_component, operation))
}

pub async fn weather(args: Vec<&str>) -> ShellCommandReturnType {
    let loading = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("p")
        .unwrap();
    loading.set_text_content(Some("Loading..."));

    // append to last element of the id raw_html

    let loading: Node = add_loading();
    let city = args[1..].join(" ");
    if city.is_empty() {
        let result_string = "Usage: weather [city]. Example: weather casablanca".to_owned();
        let operation = None;
        let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
            legacy_output: result_string,
        }));
        return Ok((output_component, operation));
    }
    let response = get_weather(city).await.unwrap();
    remove_loading(loading);
    let operation = None;
    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: response,
    }));
    Ok((output_component, operation))
}
