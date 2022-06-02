// export const useHistory = (defaultValue: Array<History>) => {
//   const [history, setHistory] = React.useState<Array<History>>(defaultValue);
//   const [command, setCommand] = React.useState<string>('');
//   const [lastCommandIndex, setLastCommandIndex] = React.useState<number>(0);

//   return {
//     history,
//     command,
//     lastCommandIndex,
//     setHistory: (value: string) =>
//       setHistory([
//         ...history,
//         {
//           id: history.length,
//           date: new Date(),
//           command,
//           output: value,
//         },
//       ]),
//     setCommand,
//     setLastCommandIndex,
//     clearHistory: () => setHistory([]),
//   };
// };

use yew::{use_state, UseStateHandle};

use crate::components::history::interface::History;


#[derive(Debug, Clone)]
pub struct UseHistoryHandle {
    history: UseStateHandle<Vec<History>>,
    command: UseStateHandle<String>,
    last_command_index :UseStateHandle<usize>,
    clear_history : |History| -> (),
    set_history: |History| -> (),
}


pub fn useHistory(defaultValue: Vec<History>) {
    let history: UseStateHandle<Vec<History>> = use_state(|| defaultValue);
    let command: UseStateHandle<String> = use_state(|| "".to_owned());
    let last_command_index: UseStateHandle<usize> = use_state(|| 0);

    let clear_history = {
        let history = history.clone();

        move |_| {
            history.set(vec![])
        }
    };

    let set_history = {
        let history = history.clone();
    
        move |new_history:History| {
            let old_history = *history;
            old_history.push(new_history);
            history.set(old_history)
        }
    };

    return {
        history;
        command;
        last_command_index;
        clear_history;
        set_history;
    }
}
