use crate::components::history::hook::use_history;
use yew::prelude::*;

#[function_component(Main)]
pub fn main() -> Html {
    let history_props = use_history();
    let index = *(history_props.last_command_index.clone());

    html! {
        <h1>{index}</h1>
    }
}
