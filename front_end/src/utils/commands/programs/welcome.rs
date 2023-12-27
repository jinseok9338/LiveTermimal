use web_sys::HtmlElement;
use yew::prelude::*;

use crate::utils::commands::commands_string::add_string_stream;

#[derive(Properties, PartialEq, Clone)]
pub struct WelcomeProps {}

#[function_component(WelcomeComponent)]
pub fn welcome(_props: &WelcomeProps) -> Html {
    let div_ref = use_node_ref();
    let div_ref_clone = div_ref.clone();

    use_effect_with((), move |_| {
        if let Some(div_element) = div_ref_clone.cast::<HtmlElement>() {
            let stream_string = r"
    Hello, fellow users! Welcome to my portfolio website. This was made with Rust and WebAssembly.
    I hope you enjoy the experience. There are plenty of commands you can try such as sumfetch, repo, and more.
    If you don't know where to begin, type help to see the list of commands. Or type 'sumfetch' to display summary.
    Any feedback is appreciated.";
            add_string_stream(stream_string, div_element);
        }
    });

    html! {
        <div ref={div_ref} class="mb-2" >
            <span class="font-bold text-3xl">{"Welcome To"}</span>
            <pre class="animate-twinkle w-32">
            { r#"
    █████        ███                       ███████████
    ░░███        ░░░                       ░█░░░███░░░█
    ░███        ████  █████ █████  ██████ ░   ░███  ░   ██████  ████████  █████████████
    ░███       ░░███ ░░███ ░░███  ███░░███    ░███     ███░░███░░███░░███░░███░░███░░███
    ░███        ░███  ░███  ░███ ░███████     ░███    ░███████  ░███ ░░░  ░███ ░███ ░███
    ░███      █ ░███  ░░███ ███  ░███░░░      ░███    ░███░░░   ░███      ░███ ░███ ░███
    ███████████ █████  ░░█████   ░░██████     █████   ░░██████  █████     █████░███ █████
    ░░░░░░░░░░░ ░░░░░    ░░░░░     ░░░░░░     ░░░░░     ░░░░░░  ░░░░░     ░░░░░ ░░░ ░░░░░
                "#}
         </pre>
    </div>
    }
}
