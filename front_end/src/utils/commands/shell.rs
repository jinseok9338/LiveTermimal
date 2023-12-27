// move shell to command Struct ...

use web_sys::Window;
use yew::UseStateHandle;

use crate::{
    components::history::{
        history_function::{clear_history, set_history},
        interface::History,
    },
    config::command_config::config::Config,
};

use super::{
    command_exists::command_exists,
    commands_context_hook::COMMAND_LIST_VEC,
    execute_command::execute_command,
    programs::{legacy::LegacyProps, programs::OutputComponent},
};

pub async fn shell(
    args: Vec<&str>,
    command_handler: UseStateHandle<String>,
    history_handler: UseStateHandle<Vec<History>>,
    window: Window,
    config: &'static Config<'static>,
) {
    let first_arg = args[0].to_lowercase();
    let command = &*command_handler;
    let command_exists = command_exists(first_arg.clone(), COMMAND_LIST_VEC.to_vec()).unwrap();
    if (&first_arg) == "clear" {
        clear_history(history_handler);
    } else if command_handler.is_empty() {
        let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
            legacy_output: "".to_owned(),
        }));
        set_history(history_handler, "".to_owned(), output_component)
    } else if !command_exists {
        let first_arg_clone = first_arg.clone();
        let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
            legacy_output: format!(
                "shell: command not found: {command}. Try 'help' to get started.",
                command = first_arg_clone
            )
            .to_owned(),
        }));
        set_history(history_handler, command.to_owned(), output_component)
    } else {
        // execute the command output
        let first_arg_clone = first_arg.clone();
        let args_clone = args.clone();
        let output = execute_command(first_arg_clone, args_clone, window, config).await;
        let output_component = output.unwrap();

        set_history(history_handler, command.to_owned(), output_component)
    }
    command_handler.set("".to_owned())
}
