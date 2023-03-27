use web_sys::Element;
use yew::prelude::*;

use crate::components::ps_1::Ps1;

#[derive(Properties, PartialEq)]
pub struct RawHtmlProps {
    pub raw_html_as_string: String,
}

#[function_component(RawHtmlWithChildren)]
pub fn raw_html(props: &RawHtmlProps) -> Html {
    let raw_html_as_string = props.raw_html_as_string.clone();
    let raw_html_as_string_clone = raw_html_as_string.clone();
    let raw_html_ref = use_node_ref();
    let raw_html_ref_clone = raw_html_ref.clone();

    use_effect_with_deps(
        move |_| {
            let html_element = raw_html_ref_clone
                .cast::<Element>()
                .expect("raw_html_ref not attached to div element");
            html_element.set_inner_html(&raw_html_as_string);
            move || {}
        },
        (),
    );

    html! {
        <div>
            <div class="flex flex-row space-x-2">
                <div class="flex-shrink">
                    <Ps1 />
                </div>
                <div class="flex-grow">{raw_html_as_string_clone}</div>
            </div>
            <p
                ref={raw_html_ref}
                class="whitespace-pre-wrap mb-2 leading-normal"
            />
        </div>
    }
}
