use crate::components::history::history_context_hook::use_history;

use super::commands::commands::{use_command, CommandsContext};

pub fn handle_tap_completion() {
    let history = use_history();
    let command_handler = history.clone().command;
    let command_context = use_command();
    let existing_commands = command_context.command_list;

    let commands = existing_commands
        .into_iter()
        .filter(|command| command.starts_with(&*command_handler))
        .collect::<Vec<&str>>();

    if commands.len() == 1 {
        command_handler.set(commands[0].to_string())
    }
}
