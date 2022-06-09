// move shell to command Struct ...

use crate::components::history::history_context_hook::HistoryContext;

use super::{command_exists::command_exists, commands_context_hook::CommandsContext};

pub fn shell(history_context: HistoryContext, commands_context: CommandsContext) {
    let command_clone = history_context.command.clone();
    let args: Vec<&str> = (*command_clone.split(" ").collect::<Vec<&str>>()).to_vec();
    let first_arg = args[0].to_lowercase();
    let command_exists = command_exists(first_arg.clone()).unwrap();

    if (&first_arg) == "clear" {
        history_context.clear_history();
    } else if *command_clone == "".to_owned() {
        history_context.set_history("".to_owned())
    } else if !command_exists {
        let first_arg_clone = first_arg.clone();
        history_context.set_history(
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
        history_context.set_history(output.unwrap())
    }
    command_clone.set("".to_owned())
}
