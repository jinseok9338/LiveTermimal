use std::ascii::AsciiExt;

use crate::components::history::hook::use_history;

use super::{bin::commands::Commands, command_exists::command_exists};

// import React from 'react';
// import * as bin from './bin';

// export const shell = async (
//   command: string,
//   setHistory: (value: string) => void,
//   clearHistory: () => void,
//   setCommand: React.Dispatch<React.SetStateAction<string>>,
// ) => {
//   const args = command.split(' ');
//   args[0] = args[0].toLowerCase();

//   if (args[0] === 'clear') {
//     clearHistory();
//   } else if (command === '') {
//     setHistory('');
//   } else if (Object.keys(bin).indexOf(args[0]) === -1) {
//     setHistory(
//       `shell: command not found: ${args[0]}. Try 'help' to get started.`,
//     );
//   } else {
//     const output = await bin[args[0]](args.slice(1));
//     setHistory(output);
//   }

//   setCommand('');
// };

pub async fn shell() {
    let history = use_history();
    let command_clone = history.command.clone();
    let args: Vec<&str> = (*command_clone.split(" ").collect::<Vec<&str>>()).to_vec();
    let arg = &args[0].to_ascii_lowercase();
    let command_exists = command_exists(arg.to_owned()).unwrap();

    if arg == "clear" {
        history.clear_history();
    } else if *command_clone == "".to_owned() {
        history.set_history("".to_owned())
    } else if !command_exists {
        let output = Commands::COMMAND_LIST;
        history.set_history(output);
    }
    command_clone.set("".to_owned())
}
