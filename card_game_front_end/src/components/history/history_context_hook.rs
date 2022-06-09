use crate::components::history::interface::History;
use instant::Instant;
use yew::{
    function_component, html, use_context, use_state, Children, ContextProvider, Properties,
    UseStateHandle,
};

#[derive(Debug, PartialEq, Clone)]
pub struct HistoryContext {
    pub history: UseStateHandle<Vec<History>>,
    pub command: UseStateHandle<String>,
    pub last_command_index: UseStateHandle<u32>,
}

impl HistoryContext {
    pub fn new(
        history: UseStateHandle<Vec<History>>,
        command: UseStateHandle<String>,
        last_command_index: UseStateHandle<u32>,
    ) -> Self {
        Self {
            history,
            command,
            last_command_index,
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
