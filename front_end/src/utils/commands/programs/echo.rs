use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct EchoProps {
    pub args: Vec<String>,
}

#[function_component(EchoComponent)]
pub fn legacy(props: &EchoProps) -> Html {
    html! {
            <div name="raw_html" class="whitespace-pre-wrap mb-2 leading-normal" />
    }
}
