use std::sync::Arc;

use wasm_bindgen::JsCast;

use web_sys::{HtmlElement, Window};
use yew::prelude::*;

use crate::components::history::history_context_hook::HistoryContext;
use crate::components::history::history_function::{clear_history, set_history};
use crate::components::history::interface::History;
use crate::components::history::keyboard_event_hanlder::{
    handle_arrow_down, handle_arrow_up, handle_enter,
};
use crate::components::ps_1::Ps1;
use crate::config::config::config::Config;
use crate::utils::commands::command_exists::command_exists;

use crate::utils::commands::commands_context_hook::CommandsContext;

use crate::utils::commands::tap_completion::handle_tap_completion;

fn on_submit_callback(
    history_handler: &UseStateHandle<Vec<History>>,
    last_command_index_handler: &UseStateHandle<u32>,
    command_handler: Arc<UseStateHandle<String>>,
    command_list: Arc<Vec<String>>, // take an owned Vec<String> instead of a reference to Vec<&str>
    container_element: &HtmlElement,
    window: &Window,
    config: &Config,
    args: Arc<Vec<String>>,
) -> Callback<KeyboardEvent> {
    let history_handler = history_handler.clone();
    let last_command_index_handler = last_command_index_handler.clone();
    let command_handler = command_handler.clone();
    
    let container_element = container_element.clone();
    let window = window.clone();
    let config = config.clone();
    let args = args.clone();
   

    Callback::from(
        move |event: KeyboardEvent| match (event.key().as_str(), event.ctrl_key()) {
            ("c", true) => {
                event.prevent_default();
                command_handler.set("".to_owned());
                set_history(&history_handler, &command_handler, "".to_owned());
                last_command_index_handler.set(0);
            }
            ("l", true) => {
                event.prevent_default();
                clear_history(&history_handler);
            }
            ("Tab", _) => {
                event.prevent_default();
                handle_tap_completion(&command_handler, command_list.to_vec());
            }
            ("Enter", _) => {
                event.prevent_default();
                handle_enter(
                    &history_handler,
                    &last_command_index_handler,
                    &command_handler,
                    command_list.to_vec(),
                    &container_element,
                    &window,
                    &config,
                    args.to_vec(),
                );
            }
            ("ArrowUp", _) => {
                event.prevent_default();
                handle_arrow_up(
                    &history_handler,
                    &last_command_index_handler,
                    &command_handler,
                );
            }
            ("ArrowDown", _) => {
                event.prevent_default();
                handle_arrow_down(
                    &history_handler,
                    &last_command_index_handler,
                    &command_handler,
                );
            }
            _ => {}
        },
    )
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub input_ref: NodeRef,
    pub container_ref: NodeRef,
    pub command_list:  Arc<Vec<String>>,
}

// Do not clone the context clone the handler... I guess...
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let input_ref = &props.input_ref;
    let history_context = use_context::<HistoryContext>().expect("no ctx found");
     let history_context = Arc::new(history_context); // Add this line

    let command_context = use_context::<CommandsContext>().expect("no ctx found");
    let container_element = props.container_ref.cast::<HtmlElement>().unwrap();



    let args_context = use_context::<HistoryContext>().expect("no ctx found");
    let args = args_context.command;
    // split the args into a vector
    let args = args.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
    let args = Arc::new(args);


    let green_or_grey = use_state_eq(|| "dark:text-dark-gray text-light-gray".to_owned());

    fn on_input_callback(
        input_ref: &HtmlElement,
        history_context: &HistoryContext,
    ) -> Callback<InputEvent> {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            let input = input_ref
                .dyn_ref::<web_sys::Element>()
                .unwrap().clone()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
        })
    }

    let command_list = props.command_list.clone();
    

    use_effect_with_deps(
        move |_| {
            if command_exists(&history_context.command, (&command_list).to_vec()) {
                green_or_grey.set("dark:text-dark-red text-light-green".to_owned())
            } else {
                green_or_grey.set("dark:text-dark-gray text-light-gray".to_owned())
            }
            || {}
        },
        [&history_context.command],
    );

    let command_list = Arc::clone(&props.command_list);
    let history_command = Arc::new(history_context.command.clone());
    let last_command_index = history_context.last_command_index.clone();

    html! {

        <div class="flex flex-row space-x-2">
              <label htmlFor="prompt" class="flex-shrink">
                <Ps1 />
              </label >
              <input
                ref={input_ref.clone()}
                id="prompt"
                type="text"

                // value={**history_command.clone()}
                oninput = {on_input_callback(input_ref.cast::<HtmlElement>().unwrap().as_ref(), &history_context)}
                class={classes!("bg-light-background", "dark:bg-dark-background", "focus:outline-none", "flex-grow", *green_or_grey)}
                autofocus={true}
                autocomplete="off"
                onkeydown={on_submit_callback(
                    &history_context.history,
                    &last_command_index,
                    history_command,
                    command_list, // this nee
                    &container_element,
                    &command_context.window,
                    &command_context.config,
                    args
                )}
                spellcheck="false"
              />

            </div>
    }
}
