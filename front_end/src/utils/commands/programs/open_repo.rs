use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

use crate::{
    components::history::history_context_hook::HistoryContext,
    config::command_config::config::Config,
};
#[derive(Properties, PartialEq, Clone)]
pub struct OpenRepoProps {
    pub config: &'static Config<'static>,
}

#[function_component(OpenRepoComponent)]
pub fn open_repo(props: &OpenRepoProps) -> Html {
    let config = props.config;
    let repo = config.repo;
    let current_index_handler: UseStateHandle<i32> = use_state(|| 0);
    let history_context = use_context::<HistoryContext>().unwrap();
    let input_ref = use_node_ref();
    let input_ref_clone = input_ref.clone();
    let current_index_handler_clone = current_index_handler.clone();
    let is_program_running_handler = history_context.is_running.clone();

    use_effect_with((), move |_| {
        let input_element = input_ref.cast::<HtmlInputElement>();

        if *current_index_handler_clone == 0 {
            is_program_running_handler.set(true);
        }

        match input_element {
            Some(element) => {
                if element.focus().is_ok() {
                    // The element was successfully focused
                } else {
                    // The element could not be focused
                }
            }
            None => {}
        }
        || {}
    });

    render_open_repo(
        repo.to_owned(),
        current_index_handler,
        history_context.clone(),
        input_ref_clone,
    )
}

fn render_open_repo(
    repo: String,
    current_index_handler: UseStateHandle<i32>,
    history_context: HistoryContext,
    input_ref: NodeRef,
) -> Html {
    let current_index = *current_index_handler.clone();
    let current_index_handler_for_closure = current_index_handler.clone();
    let input_ref_clone = input_ref.clone();
    let is_program_running_handler = history_context.is_running.clone();

    let on_submit = Callback::from(move |event: KeyboardEvent| {
        let input_element = input_ref_clone.cast::<HtmlInputElement>();
        let value = input_element.unwrap().value();

        if event.key() == "Enter" && is_value_yes(&value) {
            current_index_handler_for_closure.set(1);
            is_program_running_handler.set(false);
            window().unwrap().open_with_url(repo.as_ref()).unwrap();
        }

        if event.key() == "Enter" && !is_value_yes(&value) {
            current_index_handler_for_closure.set(2);
            is_program_running_handler.set(false);
        }

        // if ctrl + c
        if event.ctrl_key() && event.key() == "c" {
            current_index_handler_for_closure.set(2);
            is_program_running_handler.set(false)
        }
    });

    if current_index == 0 {
        return html!(
            <div class="flex flex-row space-x-2">
            <label htmlFor="prompt" class="flex-shrink">{"This action will open the repository of this project. Do you want to open the repo? [y/n] "}</label>
            <input
                ref={input_ref.clone()}
                id="prompt"
                type="text"
                class={classes!("bg-light-background", "dark:bg-dark-background", "focus:outline-none", "flex-grow")}
                autofocus=true
                autocomplete="off"
                onkeydown={on_submit}
                spellcheck="false"
            />
        </div>
        );
    }

    if current_index == 1 {
        // exit the program successfully
        return html!(
            <div class="flex flex-col">
            <span>{"This action will open the repository of this project. Do you want to open the repo? [y/n] "}</span>
            <span>{"Opening the repo"}</span>
            </div>
        );
    }
    if current_index == 2 {
        // exit the program unsuccessfully
        return html!(
            <div class="flex flex-col">
            <span>{"This action will open the repository of this project. Do you want to open the repo? [y/n] "}</span>
            <span>{"Exit the program unsuccessfully"}</span>
            </div>
        );
    }
    return html!();
}

fn is_value_yes(v: &String) -> bool {
    // Y,y yes
    if v.to_lowercase() == "y" || v.to_lowercase() == "yes" {
        return true;
    }
    false
}
