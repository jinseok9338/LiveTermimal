use yew::prelude::*;

use super::interface::History;
use crate::{components::ps_1::Ps1, utils::commands::programs::programs::render_child};

#[derive(Properties, PartialEq, Clone)]
pub struct RawHtmlProps {
    pub history: History,
}

#[function_component(RawHtml)]
pub fn raw_html(props: &RawHtmlProps) -> Html {
    let history = props.history.clone();

    let raw_html_ref = use_node_ref();

    html! {
        <div>
            <div class="flex flex-row space-x-2">
                <div class="flex-shrink"><Ps1 /></div>
                <div class="flex-grow" >{ &*(history.command.clone()) }</div>
            </div>
            // this is where render_component goes
            <div ref={raw_html_ref} name="raw_html" class="whitespace-pre-wrap mb-2 leading-normal" />
            { render_child(&history.output) }
        </div>
    }
}
