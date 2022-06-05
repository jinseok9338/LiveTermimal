use std::io::Error;

pub fn command_exists(command: String) -> Result<bool, Error> {
    // import enum and add "clear"
    let mut split_string = command.split(" ").collect::<Vec<&str>>();
    let command = split_string[0];
    if commands.contains(command) {
        Ok(true)
    } else {
        Ok(false)
    }
}
