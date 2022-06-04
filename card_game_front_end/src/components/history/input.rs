// import React from 'react';
// import { commandExists } from '../utils/commandExists';
// import { shell } from '../utils/shell';
// import { handleTabCompletion } from '../utils/tabCompletion';
// import { Ps1 } from './Ps1';

// export const Input = ({
//   inputRef,
//   containerRef,
//   command,
//   history,
//   lastCommandIndex,
//   setCommand,
//   setHistory,
//   setLastCommandIndex,
//   clearHistory,
// }) => {
//   const onSubmit = async (event: React.KeyboardEvent<HTMLInputElement>) => {
//     const commands: [string] = history
//       .map(({ command }) => command)
//       .filter((command: string) => command);

//     if (event.key === 'c' && event.ctrlKey) {
//       event.preventDefault();
//       setCommand('');
//       setHistory('');
//       setLastCommandIndex(0);
//     }

//     if (event.key === 'l' && event.ctrlKey) {
//       event.preventDefault();
//       clearHistory();
//     }

//     if (event.key === 'Tab') {
//       event.preventDefault();
//       handleTabCompletion(command, setCommand);
//     }

//     if (event.key === 'Enter' || event.code === '13') {
//       event.preventDefault();
//       setLastCommandIndex(0);
//       await shell(command, setHistory, clearHistory, setCommand);
//       containerRef.current.scrollTo(0, containerRef.current.scrollHeight);
//     }

//     if (event.key === 'ArrowUp') {
//       event.preventDefault();
//       if (!commands.length) {
//         return;
//       }
//       const index: number = lastCommandIndex + 1;
//       if (index <= commands.length) {
//         setLastCommandIndex(index);
//         setCommand(commands[commands.length - index]);
//       }
//     }

//     if (event.key === 'ArrowDown') {
//       event.preventDefault();
//       if (!commands.length) {
//         return;
//       }
//       const index: number = lastCommandIndex - 1;
//       if (index > 0) {
//         setLastCommandIndex(index);
//         setCommand(commands[commands.length - index]);
//       } else {
//         setLastCommandIndex(0);
//         setCommand('');
//       }
//     }
//   };

//   const onChange = ({
//     target: { value },
//   }: React.ChangeEvent<HTMLInputElement>) => {
//     setCommand(value);
//   };

//   return (
//     <div className="flex flex-row space-x-2">
//       <label htmlFor="prompt" className="flex-shrink">
//         <Ps1 />
//       </label>

//       <input
//         ref={inputRef}
//         id="prompt"
//         type="text"
//         className={`bg-light-background dark:bg-dark-background focus:outline-none flex-grow ${
//           commandExists(command) || command === ''
//             ? 'text-dark-green'
//             : 'text-dark-red'
//         }`}
//         value={command}
//         onChange={onChange}
//         autoFocus
//         onKeyDown={onSubmit}
//         autoComplete="off"
//         spellCheck="false"
//       />
//     </div>
//   );
// };

use yew::prelude::*;

use crate::components::history::{hook::use_history, interface::History};
use crate::components::ps_1::Ps1;

#[derive(Properties, PartialEq)]
pub struct InputProps {}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let history = use_history();

    let on_submit = {
        let cloned_history = history.clone();
        let commands_hostory = history.clone().history;
        let commands_history_vec = &(*commands_hostory);
        let commands = commands_history_vec
            .into_iter()
            .map(|history| &history.command)
            .collect::<Vec<&String>>();

        move |event: KeyboardEvent| {
            if event.key() == "c".to_owned() && event.ctrl_key() {
                event.prevent_default();
                cloned_history.command.set("".to_owned());
                cloned_history.set_history("".to_owned());
                cloned_history.last_command_index.set(0);
            }

            if event.key() == "l".to_owned() && event.ctrl_key() {
                event.prevent_default();
                cloned_history.clear_history();
            }

            if event.key() == "Tab".to_owned() {
                event.prevent_default();
                todo! {} //handle_tab_completion(command, cloned_history.command.set);
            }

            if event.key() == "Enter".to_owned() || event.code() == "13".to_owned() {
                event.prevent_default();
                cloned_history.last_command_index.set(0);
                // await shell(command, setHistory, clearHistory, setCommand);
                // containerRef.current.scrollTo(0, containerRef.current.scrollHeight);
            }

            todo! {}
        }
    };

    let on_change = {
        let cloned_history = history.clone();

        |input_event: InputEvent| cloned_history.command.set(input_event.data().unwrap())
    };

    html! {
        <div className="flex flex-row space-x-2">
              <label htmlFor="prompt" className="flex-shrink">
                <Ps1 />
              </label>

              <input
                ref={inputRef}
                id="prompt"
                type="text"
                value={command}
                onChange={onChange}
                autoFocus="true"
                onKeyDown={onSubmit}
                autoComplete="off"
                spellCheck="false"
              />
            </div>
    }
}
