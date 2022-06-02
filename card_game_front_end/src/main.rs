mod components;
mod config;

use gloo_console::log;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let input_ref = use_node_ref();

    let onclick = {
        let input_ref = input_ref.clone();

        move |_| {
            let input = input_ref.cast::<InputElement>();
            input.unwrap().focus().unwrap()
        }
    };
    html! {

    <div
      class="text-light-foreground dark:text-dark-foreground min-w-max text-xs md:min-w-full md:text-base"
    >
      <main class="bg-light-background dark:bg-dark-background w-full h-full p-2">
        <button {onclick}>{"Hello World"}</button>
      </main>
    </div>}
}

fn main() {
    yew::start_app::<App>();
}
