#![allow(clippy::all)]
use std::sync::Arc;

use yew::UseStateHandle;

pub fn handle_tap_completion(
    command_handler: &UseStateHandle<String>,
    command_list: Vec<String>,
) {
    let commands: Vec<&str> = command_list
    
        .iter()
        .filter(|command| command.starts_with(&**command_handler))
        .map(|command| command.as_str())
        .collect();

    if commands.len() == 1 {
        command_handler.set(commands[0].to_string());
    }
}
