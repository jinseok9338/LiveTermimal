use crate::components::history::hook::use_history;
use yew::prelude::*;

#[function_component(Main)]
pub fn main() -> Html {
    let history_props = use_history();
    let print = *(history_props.history);
    html! {
        <h1>{"command"}</h1>
    }
}
