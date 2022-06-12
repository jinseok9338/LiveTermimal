use web_sys::{HtmlElement, HtmlInputElement, ScrollToOptions};
use yew::prelude::*;

use crate::components::history::history_context_hook::use_history;
use crate::components::history::history_function::{clear_history, set_history};
use crate::components::ps_1::Ps1;
use crate::utils::commands::command_exists::command_exists;
use crate::utils::commands::commands_context_hook::use_command;
use crate::utils::commands::shell::shell;
use crate::utils::commands::tap_completion::handle_tap_completion;

use gloo_console::log;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub input_ref: NodeRef,
    pub container_ref: NodeRef,
}
// Do not clone the context clone the handler... I guess...
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let input_ref = &props.input_ref;
    let history_context = use_history();
    let green_or_grey = use_state_eq(|| "dark:text-dark-gray text-light-gray".to_owned());

    let command_context = use_command();
    let command_list = command_context.command_list.clone();
    let command_handler = history_context.command.clone();
    let current_command = &*(history_context.command.clone());
    let current_command_value_for_input = &*(history_context.command.clone());
    let container_element = props.container_ref.cast::<HtmlElement>().unwrap();

    let green_or_gray_class = &*green_or_grey.clone();

    let on_submit = {
        // history_context
        let history_handler = history_context.history.clone();
        let last_command_index_handler = history_context.last_command_index.clone();

        //command_context
        let command_list = command_context.command_list.clone();
        let window = command_context.window;
        let config = command_context.config;
        let on_submit_command = history_context.command.clone();
        //input_value_handler
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "c".to_owned() && event.ctrl_key() {
                event.prevent_default();
                on_submit_command.set("".to_owned());
                set_history(
                    history_handler.clone(),
                    on_submit_command.clone(),
                    "".to_owned(),
                );
                last_command_index_handler.set(0);
            };

            if event.key() == "l".to_owned() && event.ctrl_key() {
                event.prevent_default();
                clear_history(history_handler.clone());
            };

            if event.key() == "Tab".to_owned() {
                event.prevent_default();
                handle_tap_completion(on_submit_command.clone(), command_list.clone());
            };

            if event.key() == "Enter".to_owned() {
                event.prevent_default();
                last_command_index_handler.set(0);

                let args: Vec<&str> =
                    ((&*on_submit_command).split(" ").collect::<Vec<&str>>()).to_vec(); // This is the problem...

                shell(
                    args,
                    on_submit_command.clone(),
                    history_handler.clone(),
                    window.clone(),
                    config.clone(),
                    command_list.clone(),
                );
                container_element.scroll_to_with_scroll_to_options(
                    &ScrollToOptions::new()
                        .left(0.try_into().unwrap())
                        .top(container_element.scroll_height().try_into().unwrap()),
                );
                on_submit_command.set("".to_owned());
            };

            if event.key() == "ArrowUp" {
                event.prevent_default();
                let commands_history = &*(history_handler.clone());

                let commands = commands_history
                    .into_iter()
                    .map(|history| *history.command.clone())
                    .collect::<Vec<String>>();

                let command_length = commands.len() as u32;

                if command_length == (0 as u32) {
                    return;
                };

                let index = *(last_command_index_handler) + 1;

                if index <= command_length.clone().try_into().unwrap() {
                    last_command_index_handler.set(index);
                    on_submit_command.set(
                        (*commands[(&command_length - &index) as usize].to_owned()).to_string(),
                    )
                };
            };
            if event.key() == "ArrowDown" {
                event.prevent_default();
                let commands_history = &*(history_handler.clone());
                let commands = commands_history
                    .into_iter()
                    .map(|history| *history.command.clone())
                    .collect::<Vec<String>>();

                let command_length = commands.len();
                if command_length == 0 {
                    return;
                }
                let index = *(last_command_index_handler) - 1;
                if index > 0 {
                    last_command_index_handler.set(index);

                    on_submit_command
                        .set((*commands[&command_length - index as usize].to_owned()).to_string())
                } else {
                    last_command_index_handler.set(0);
                    on_submit_command.set("".to_owned());
                }
            }
        })
    };

    let oninput = {
        let input_ref = input_ref.clone();
        let on_onchange_command = history_context.command.clone();
        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            on_onchange_command.set(input.value());
        })
    };

    use_effect_with_deps(
        move |_| {
            let green_or_grey = green_or_grey.clone();
            if command_exists((*&command_handler.clone()).to_string(), command_list).unwrap() {
                green_or_grey.set("dark:text-dark-green text-light-green".to_owned())
            } else {
                green_or_grey.set("dark:text-dark-gray text-light-gray".to_owned())
            }
            || {}
        },
        [current_command.to_string()],
    );

    html! {

        <div class="flex flex-row space-x-2">
              <label htmlFor="prompt" class="flex-shrink">
                <Ps1 />
              </label >
              <input
                ref={input_ref.clone()}
                id="prompt"
                type="text"
                // need to have value soon.. .but it's pretty much hopeless
                value={current_command_value_for_input.to_string()}
                {oninput}
                class={classes!("bg-light-background", "dark:bg-dark-background", "focus:outline-none", "flex-grow", green_or_gray_class)} // if the command exist show color green
                autofocus={true}
                autocomplete="off"
                onkeydown={on_submit}
                spellcheck="false"
              />

            </div>
    }
}
