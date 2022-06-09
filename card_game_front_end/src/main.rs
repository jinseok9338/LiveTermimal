mod components;
mod config;
mod index;
mod utils;
use crate::components::history::history_context_hook::HistoryContextProvider;
use crate::utils::commands::commands_context_hook::CommandContextProvider;
// use gloo_console::log;
use index::Index;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let input_ref = use_node_ref();

    let onclick = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<InputElement>().unwrap();
            input.focus().unwrap()
        })
    };

    html! {
      <CommandContextProvider>
        <HistoryContextProvider>
          <div
              class="text-light-foreground dark:text-dark-foreground min-w-max text-xs md:min-w-full md:text-base"
              onclick={onclick}
            >
            <main class="bg-light-background dark:bg-dark-background w-full h-full p-2" >
              <Index input_ref ={input_ref}/>
            </main>
          </div>
        </HistoryContextProvider>
      </CommandContextProvider>
    }
}

fn main() {
    yew::start_app::<App>();
}
