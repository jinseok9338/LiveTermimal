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

#[derive(Debug)]
pub struct UseHistoryHandle<T, A>
where
    T: Fn(History) -> (),
    A: Fn(String) -> (),
{
    history: UseStateHandle<Vec<History>>,
    command: UseStateHandle<String>,
    last_command_index: UseStateHandle<usize>,
    clear_history: A,
    set_history: T,
}

pub fn useHistory(defaultValue: Vec<History>) {
    let history: UseStateHandle<Vec<History>> = use_state(|| defaultValue);
    let command: UseStateHandle<String> = use_state(|| "".to_owned());
    let last_command_index: UseStateHandle<usize> = use_state(|| 0);

    let clear_history = {
        let history = history.clone();

        move |_: String| history.set(vec![])
    };

    let set_history = {
        let history = history.clone();
        let mut old_history = *history;

        move |new_history: History| {
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
    };
}
