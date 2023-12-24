use crate::components::history::history_context_hook::HistoryContext;
use crate::components::history::raw_html::RawHtml;
use yew::prelude::*;

#[function_component(HistoryComponent)]
pub fn history_component() -> Html {
    let history = use_context::<HistoryContext>().unwrap();
    let history_cloned = &*(history.clone().history);
    html! {
        <ul name="history_list">
            { history_cloned.iter().enumerate().map(|(index, history)| {
                let is_last = index == history_cloned.len() - 1;
                html!{<RawHtml key={index} history={history.clone()} is_last={is_last} />}
            }).collect::<Html>() }
        </ul>
    }
}
