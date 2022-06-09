// move shell to command Struct ...
use yew::UseStateHandle;

use crate::components::history::history_context_hook::use_history;

use super::{command_exists::command_exists, commands::commands::use_command};

pub fn shell(command_clone: UseStateHandle<String>) {
    let args: Vec<&str> = (*command_clone.split(" ").collect::<Vec<&str>>()).to_vec();
    let first_arg = args[0].to_lowercase();
    let command_exists = command_exists(first_arg.clone()).unwrap();

    if (&first_arg) == "clear" {
        history.clear_history();
    } else if *command_clone == "".to_owned() {
        history.set_history("".to_owned())
    } else if !command_exists {
        let first_arg_clone = first_arg.clone();
        history.set_history(
            format!(
                "shell: command not found: {command}. Try 'help' to get started.",
                command = first_arg_clone
            )
            .to_owned(),
        )
    } else {
        // execute the command output
        let first_arg_clone = first_arg.clone();
        let args_clone = args.clone();
        let output = commands_context.execute_command(first_arg_clone, args_clone);
        history.set_history(output.unwrap())
    }
    command_clone.set("".to_owned())
}
