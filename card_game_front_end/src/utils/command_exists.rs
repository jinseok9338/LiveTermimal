use std::io::Error;

use super::bin::commands::Commands;

pub fn command_exists(command: String) -> Result<bool, Error> {
    let command_struct = Commands::COMMAND_LIST;
    let mut split_string = command.split(" ").collect::<Vec<&str>>();
    let command = split_string[0];
    if command_struct.contains(&command.to_owned()) {
        Ok(true)
    } else {
        Ok(false)
    }
}
