use web_sys::Element;
use yew::prelude::*;

use super::interface::History;
use crate::components::ps_1::Ps1;

#[derive(Properties, PartialEq)]
pub struct RawHtmlProps {
    pub history: History,
}

#[function_component(RawHtml)]
pub fn raw_html(props: &RawHtmlProps) -> Html {
    let history = props.history.clone();
    let raw_html_ref = use_node_ref();

    {
        let raw_html_ref = raw_html_ref.clone();

        use_effect_with_deps(
            move |_| {
                let html_element = raw_html_ref
                    .cast::<Element>()
                    .expect("raw_html_refnot attached to div element");
                html_element.set_inner_html(&history.output);
                move || {}
            },
            (),
        );
    }

    html! {
        <div key={&*(history.command.clone())}>
        <div class="flex flex-row space-x-2">
          <div class="flex-shrink">
            <Ps1 />
          </div>

          <div class="flex-grow">{&*(history.command.clone())}</div>
        </div>

        <p
          ref ={raw_html_ref}
          class="whitespace-pre-wrap mb-2"
        />
        <p>{"this is weird"}</p>
      </div>
    }
}
