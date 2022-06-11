// move shell to command Struct ...

use instant::Instant;
use web_sys::Window;
use yew::UseStateHandle;

use crate::{
    components::history::{
        history_function::{clear_history, set_history},
        interface::History,
    },
    config::config::config::Config,
};

use super::{command_exists::command_exists, execute_command::execute_command};

pub fn shell(
    args: Vec<&str>,
    command_handler: UseStateHandle<String>,
    history_handler: UseStateHandle<Vec<History>>,
    window: Window,
    config: Config,
    command_list: Vec<&'static str>,
) {
    let first_arg = args[0].to_lowercase();
    let command_exists = command_exists(first_arg.clone(), command_list.clone()).unwrap();

    if (&first_arg) == "clear" {
        clear_history(history_handler);
    } else if *command_handler == "".to_owned() {
        set_history(history_handler, command_handler.clone(), "".to_owned())
    } else if !command_exists {
        let first_arg_clone = first_arg.clone();
        set_history(
            history_handler,
            command_handler.clone(),
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
        let output = execute_command(
            first_arg_clone,
            args_clone,
            window,
            config,
            command_list.clone(),
        );
        set_history(history_handler, command_handler.clone(), output.unwrap())
    }
    command_handler.set("".to_owned())
}
