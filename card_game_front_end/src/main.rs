mod components;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let input_ref = use_node_ref();

    let onclick = {
        let input_ref = input_ref.clone();
        move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.focus()
            } else {
                println!("ds")
            }
        }
    };
    html! {
    <div
      class="text-light-foreground dark:text-dark-foreground min-w-max text-xs md:min-w-full md:text-base"
      onclick={onclick}
    >
      <main class="bg-light-background dark:bg-dark-background w-full h-full p-2">
        <h1>{"Hello World"}</h1>
      </main>
    </div>}
}

fn main() {
    yew::start_app::<App>();
}
