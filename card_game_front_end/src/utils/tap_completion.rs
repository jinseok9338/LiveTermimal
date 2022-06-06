use crate::components::history::hook::use_history;

use super::bin::commands::CommandsContext;

// import * as bin from './bin';

// export const handleTabCompletion = (
//   command: string,
//   setCommand: React.Dispatch<React.SetStateAction<string>>,
// ) => {
//   const commands = Object.keys(bin).filter((entry) =>
//     entry.startsWith(command),
//   );

//   if (commands.length === 1) {
//     setCommand(commands[0]);
//   }
// };

pub fn handle_tap_completion() {
    let history = use_history();
    let command_handler = history.clone().command;
    let existing_commands = CommandsContext::COMMAND_LIST;

    let commands = existing_commands
        .into_iter()
        .filter(|command| command.starts_with(&*command_handler))
        .collect::<Vec<String>>();

    if commands.len() == 1 {
        command_handler.set(commands[0])
    }
}
