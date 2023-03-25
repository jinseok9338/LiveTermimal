
use crate::components::history::{raw_html::RawHtml, history_context_hook::HistoryContext};
use yew::prelude::*;

#[function_component(HistoryComponent)]
pub fn history_compoenet() -> Html {
    let history = use_context::<HistoryContext>().expect("no ctx found");
    let history_cloned = &*(history.clone().history);

    html! {
        <ul>
            {
            history_cloned.to_owned().into_iter().map(|history| {
            html!{<RawHtml key={&*(history.command.clone())} history= {history.clone()}/>}
            }).collect::<Html>()
        }
        </ul>
    }
}
