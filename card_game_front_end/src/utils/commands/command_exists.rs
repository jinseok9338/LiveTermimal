pub fn command_exists(first_arg: &str, command_list: Vec<String>) -> bool {
    let split_args = first_arg.split(' ').collect::<Vec<&str>>();
    let command = split_args[0];
    command_list.contains(&command.to_owned())
}
