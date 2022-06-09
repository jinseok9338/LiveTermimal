use std::io::Error;

use super::commands::commands::use_command;

pub fn command_exists(command: String) -> Result<bool, Error> {
    let command_context = use_command();
    let command_list = command_context.command_list;

    let split_string = command.split(" ").collect::<Vec<&str>>();
    let command = split_string[0];
    if command_list.contains(&command) {
        Ok(true)
    } else {
        Ok(false)
    }
}
