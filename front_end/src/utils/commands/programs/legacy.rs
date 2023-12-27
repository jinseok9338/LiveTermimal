use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LegacyProps {
    pub legacy_output: String,
}

#[function_component(LegacyComponent)]
pub fn legacy(props: &LegacyProps) -> Html {
    let raw_html_ref = use_node_ref();

    {
        let raw_html_ref = raw_html_ref.clone();
        let output = props.legacy_output.clone();

        use_effect_with((), move |_| {
            let html_element = raw_html_ref
                .cast::<Element>()
                .expect("raw_html_refnot attached to div element");
            html_element.set_inner_html(&output);

            move || {}
        });
    }

    html! {
            <div ref={raw_html_ref} name="raw_html" class="whitespace-pre-wrap mb-2 leading-normal" />
    }
}
