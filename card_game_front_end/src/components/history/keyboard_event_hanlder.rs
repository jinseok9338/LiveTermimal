

use std::sync::Arc;

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;

use web_sys::{HtmlElement, ScrollToOptions, Window};
use yew::UseStateHandle;

use crate::{config::config::config::Config, utils::commands::shell::shell};

use super::interface::History;

pub fn handle_enter(
    history_handler: &UseStateHandle<Vec<History>>,
    last_command_index_handler: &UseStateHandle<u32>,
    command_handler:  Arc<UseStateHandle<String>>,
    command_list: Vec<String>,
    container_element: &HtmlElement,
    window: &Window,
    config: &Config,
    args: Vec<String>,
) {
    last_command_index_handler.set(0);

    let command_list = command_list.to_vec();

    let history_handler = history_handler.clone();
    let window = window.clone();
    let config = config.clone();
    let args = args.clone();

    let command_handler = command_handler.clone();
    let history_handler = history_handler.clone();

    let future = async move {
        shell(
            args,
            &command_handler,
            &history_handler,
            &window,
            &config,
            command_list,
        )
        .await;
        Ok(JsValue::NULL)
    };

    let promise = future_to_promise(future);

    container_element.scroll_to_with_scroll_to_options(
        ScrollToOptions::new()
            .left(0.try_into().unwrap())
            .top(container_element.scroll_height().try_into().unwrap()),
    );
}
pub fn handle_arrow_up(
    history_handler: &UseStateHandle<Vec<History>>,
    last_command_index_handler: &UseStateHandle<u32>,
    command_handler: &UseStateHandle<String>,
) {
    let commands_history = history_handler;
    let commands = commands_history
        .iter()
        .map(|history| history.command.clone())
        .collect::<Vec<String>>();

    let command_length = commands.len() as u32;

    if command_length == 0 {
        return;
    }

    let index = **(last_command_index_handler) + 1;

    if index <= command_length {
        last_command_index_handler.set(index);
        command_handler.set((*commands[(command_length - index) as usize].to_owned()).to_string())
    };
}

pub fn handle_arrow_down(
    history_handler: &UseStateHandle<Vec<History>>,
    last_command_index_handler: &UseStateHandle<u32>,
    command_handler: &UseStateHandle<String>,
) {
    let commands_history = history_handler;
    let commands = commands_history
        .iter()
        .map(|history| history.command.clone())
        .collect::<Vec<String>>();

    let command_length = commands.len();
    if command_length == 0 {
        return;
    }

    let index = **(last_command_index_handler) - 1_u32;
    if index > 0 {
        last_command_index_handler.set(index);

        command_handler.set((*commands[command_length - index as usize].to_owned()).to_string())
    } else {
        last_command_index_handler.set(0);
        command_handler.set("".to_owned());
    }
}
