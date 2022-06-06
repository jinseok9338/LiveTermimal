use crate::components::history::hook::use_history;

use super::bin::commands::CommandsContext;

pub fn handle_tap_completion() {
    let history = use_history();
    let command_handler = history.clone().command;
    let existing_commands = CommandsContext::COMMAND_LIST;

    let commands = existing_commands
        .into_iter()
        .filter(|command| command.starts_with(&*command_handler))
        .collect::<Vec<String>>();

    if commands.len() == 1 {
        command_handler.set(commands[0])
    }
}
