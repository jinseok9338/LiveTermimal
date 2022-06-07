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
    let el = raw_html_ref.cast::<Element>().unwrap();

    use_effect_with_deps(
        move |_| {
            {
                el.set_inner_html(&history.output);
            }
            || {}
        },
        (),
    );

    html! {
        <div key={&*(history.command.clone())}>
        <div class="flex flex-row space-x-2">
          <div class="flex-shrink">
            <Ps1 />
          </div>

          <div class="flex-grow">{&*(history.command.clone())}</div>
        </div>

        <p
          ref ={raw_html_ref.clone()}
          class="whitespace-pre-wrap mb-2"
        />
        <p>{"this is weird"}</p>
      </div>
    }
}
