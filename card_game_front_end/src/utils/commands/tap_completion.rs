#![allow(clippy::all)]
use yew::UseStateHandle;

pub fn handle_tap_completion(command_handler: &UseStateHandle<String>, command_list: Vec<&str>) {
    let commands = command_list
        .into_iter()
        .filter(|command| command.starts_with(&**command_handler))
        .collect::<Vec<&str>>();

    if commands.len() == 1 {
        command_handler.set(commands[0].to_string());
    }
}
