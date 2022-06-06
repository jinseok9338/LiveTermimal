use std::io::Error;

use super::bin::commands::CommandsContext;

pub fn command_exists(command: String) -> Result<bool, Error> {
    let command_list = CommandsContext::COMMAND_LIST;

    let split_string = command.split(" ").collect::<Vec<&str>>();
    let command = split_string[0];
    if command_list.contains(&command) {
        Ok(true)
    } else {
        Ok(false)
    }
}
