use crate::components::history::interface::History;
use yew::{
    function_component, html, use_state, Children, ContextProvider, Html, Properties,
    UseStateHandle,
};

#[derive(PartialEq, Clone)]
pub struct HistoryContext {
    pub history: UseStateHandle<Vec<History>>,
    pub command: UseStateHandle<String>,
    pub last_command_index: UseStateHandle<u32>,
    pub is_running: UseStateHandle<bool>,
}

impl HistoryContext {
    pub fn new(
        history: UseStateHandle<Vec<History>>,
        command: UseStateHandle<String>,
        last_command_index: UseStateHandle<u32>,
        is_running: UseStateHandle<bool>,
    ) -> Self {
        Self {
            history,
            command,
            last_command_index,
            is_running,
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct HistoryProviderProps {
    pub children: Children,
}

#[function_component(HistoryContextProvider)]
pub fn history_provider(props: &HistoryProviderProps) -> Html {
    let history = use_state(|| vec![].to_owned());
    let command = use_state(|| "".to_owned());
    let last_command_index = use_state(|| 0);
    let is_running_handler: UseStateHandle<bool> = use_state(|| false);

    let history_ctx = HistoryContext::new(history, command, last_command_index, is_running_handler);

    html! {
        <ContextProvider<HistoryContext> context={history_ctx}>{ props.children.clone() }</ContextProvider<HistoryContext>>
    }
}
