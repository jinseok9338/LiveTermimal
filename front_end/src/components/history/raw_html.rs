use web_sys::Element;
use yew::prelude::*;

use super::interface::History;
use crate::components::{history::interface::handle_operation, ps_1::Ps1};

#[derive(Properties, PartialEq)]
pub struct RawHtmlProps {
    pub history: History,
    pub is_last: bool,
}

#[function_component(RawHtml)]
pub fn raw_html(props: &RawHtmlProps) -> Html {
    let history = props.history.clone();
    let raw_html_ref = use_node_ref();

    {
        let raw_html_ref = raw_html_ref.clone();
        let is_last = props.is_last;

        use_effect_with((), move |_| {
            let html_element = raw_html_ref
                .cast::<Element>()
                .expect("raw_html_refnot attached to div element");
            html_element.set_inner_html(&history.output);

            let operation = history.operation.clone();

            if operation.is_some() && is_last {
                handle_operation(operation.unwrap());
            }

            move || {}
        });
    }

    html! {
        <div>
            <div class="flex flex-row space-x-2">
                <div class="flex-shrink"><Ps1 /></div>
                <div class="flex-grow" >{ &*(history.command.clone()) }</div>
            </div>
            <div ref={raw_html_ref} name="raw_html" class="whitespace-pre-wrap mb-2 leading-normal" />
        </div>
    }
}
