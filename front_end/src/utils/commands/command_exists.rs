use std::io::Error;

pub fn command_exists(first_arg: String, command_list: Vec<&str>) -> Result<bool, Error> {
    let split_args = first_arg.split(" ").collect::<Vec<&str>>();
    let command = split_args[0];
    if command_list.contains(&command) {
        Ok(true)
    } else {
        Ok(false)
    }
}
