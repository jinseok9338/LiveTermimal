mod components;
mod config;
mod index;
mod utils;
use crate::components::history::hook::HistoryContextProvider;
use crate::utils::bin::commands::CommandContextProvider;
// use gloo_console::log;
use index::Main;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let input_ref = use_node_ref();

    let onclick = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<InputElement>();
            input.unwrap().focus().unwrap()
        })
    };

    html! {
      <CommandContextProvider>
        <HistoryContextProvider default_value ={vec![]}>
          <div
              class="text-light-foreground dark:text-dark-foreground min-w-max text-xs md:min-w-full md:text-base"
              onclick={onclick}
            >
            <main class="bg-light-background dark:bg-dark-background w-full h-full p-2">
              <Main input_ref ={(&input_ref).clone()}/>
            </main>
          </div>
        </HistoryContextProvider>
      </CommandContextProvider>
    }
}

fn main() {
    yew::start_app::<App>();
}
