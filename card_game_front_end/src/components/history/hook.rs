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

use yew::{
    function_component, html, use_context, use_state, Children, ContextProvider, Properties,
    UseStateHandle,
};

use crate::components::history::interface::History;

#[derive(Debug, PartialEq, Clone)]
pub struct HistoryContext {
    pub history: UseStateHandle<Vec<History>>,
    pub command: UseStateHandle<String>,
    pub last_command_index: UseStateHandle<usize>,
}

impl HistoryContext {
    pub fn new(
        history: UseStateHandle<Vec<History>>,
        command: UseStateHandle<String>,
        last_command_index: UseStateHandle<usize>,
    ) -> Self {
        Self {
            history,
            command,
            last_command_index,
        }
    }

    pub fn clear_history(history: UseStateHandle<Vec<History>>) {
        let cloned_history = history.clone();
        let empty_vector = Vec::new();
        cloned_history.set(empty_vector)
    }

    pub fn set_history(history: UseStateHandle<Vec<History>>) {
        let cloned_history = history.clone();
        let mut old_history = (*history).clone();
        cloned_history.set(old_history)
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct HistoryProviderProps {
    pub children: Children,
    pub default_value: Vec<History>,
}

#[function_component(HistoryContextProvider)]
pub fn history_provider(props: &HistoryProviderProps) -> Html {
    let history = use_state(|| props.default_value.to_owned());
    let command = use_state(|| "".to_owned());
    let last_command_index = use_state(|| 0);

    let history_ctx = HistoryContext::new(history, command, last_command_index);

    html! {
        <ContextProvider<HistoryContext> context={history_ctx}>
            {props.children.clone()}
        </ContextProvider<HistoryContext>>
    }
}

pub fn use_history() -> HistoryContext {
    use_context::<HistoryContext>().expect("no ctx found")
}
