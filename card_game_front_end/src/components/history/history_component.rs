use crate::components::history::raw_html::RawHtml;
use crate::components::{history::hook::use_history, ps_1::Ps1};
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryComponentProps {}

#[function_component(HistoryComponent)]
pub fn history_compoenet(props: &HistoryComponentProps) -> Html {
    let history = use_history();
    let history_cloned = &*(history.clone().history);

    html! {
        <>
            {history_cloned.to_owned().into_iter().map(|history| {
              html!{<RawHtml history= {history.clone()}/>}
            }).collect::<Html>()}
        </>
    }
}
