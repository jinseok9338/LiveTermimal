use std::io::Error;

use super::bin::commands::{use_command, CommandsContext};

pub fn command_exists(command: String) -> Result<bool, Error> {
    let command_context = use_command();
    let command_list = CommandsContext::COMMAND_LIST;

    let mut split_string = command.split(" ").collect::<Vec<&str>>();
    let command = split_string[0];
    if command_list.contains(&command.to_owned()) {
        Ok(true)
    } else {
        Ok(false)
    }
}
