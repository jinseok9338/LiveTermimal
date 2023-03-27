// move shell to command Struct ...

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

pub async fn shell(
    args: Vec<String>,
    command_handler: &UseStateHandle<String>,
    history_handler: &UseStateHandle<Vec<History>>,
    window: &Window,
    config: &Config,
    command_list: Vec<String>,
) {
    let first_arg = args[0].to_lowercase();
    let command_exists = command_exists(&first_arg, command_list.clone());

    if (&first_arg) == "clear" {
        clear_history(&history_handler);
    } else if command_handler.is_empty() {
        set_history(&history_handler, &command_handler, String::new())
    } else if !command_exists {
        let first_arg_clone = first_arg.clone();
        set_history(
            &history_handler,
            &command_handler,
            format!("shell: command not found: {first_arg_clone}. Try 'help' to get started.",),
        );
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
        )
        .await;

        set_history(&history_handler, &command_handler, output.unwrap());
    }
    command_handler.set(String::new())
}
