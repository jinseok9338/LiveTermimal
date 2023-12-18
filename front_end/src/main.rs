mod components;
mod config;
mod index;
mod utils;
use crate::components::history::history_context_hook::HistoryContextProvider;

// use gloo_console::log;
use index::Index;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let input_ref = use_node_ref();

    html! {
            <HistoryContextProvider>
                <html class="dark" id="theme">
                    // for manually changing the dark theme and light theme
                    <div
                        class="text-light-foreground dark:text-dark-foreground bg-light-background dark:bg-dark-background min-w-max min-h-max text-xs md:min-w-full md:min-h-full md:text-base transition duration-300"
                    >
                        <main
                            class="bg-light-background dark:bg-dark-background w-full h-full p-2 transition duration-300"
                        ><Index input_ref={input_ref} /></main>
                    </div>
                </html>
            </HistoryContextProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
