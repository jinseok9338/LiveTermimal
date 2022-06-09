use web_sys::{HtmlElement, HtmlInputElement, ScrollToOptions};
use yew::prelude::*;

use crate::components::history::history_context_hook::use_history;
use crate::components::ps_1::Ps1;
use crate::utils::shell::shell;
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
    let input_value = use_state(|| "".to_owned());
    let on_submit_command = history_context.command.clone();
    let on_change_command = history_context.command.clone();

    let on_submit = {
        let history = history_context.history.clone();
        let last_command_index = history_context.last_command_index.clone();
        let container_element = props.container_ref.cast::<HtmlElement>().unwrap();
        let input_value = input_value.clone();

        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "c".to_owned() && event.ctrl_key() {
                event.prevent_default();
                on_submit_command.set("".to_owned());
                history_context.set_history("".to_owned());
                last_command_index.set(0);
            }

            if event.key() == "l".to_owned() && event.ctrl_key() {
                event.prevent_default();
                history_context.clear_history();
            }

            if event.key() == "Tab".to_owned() {
                event.prevent_default();
                todo! {} //handle_tab_completion(command, cloned_history.command.set);
            }

            if event.key() == "Enter".to_owned() {
                event.prevent_default();
                last_command_index.set(0);
                // let args: Vec<&str> = (*command_clone.split(" ").collect::<Vec<&str>>()).to_vec();
                shell(args); // pass history and command as args... Darn it...
                container_element.scroll_to_with_scroll_to_options(
                    &ScrollToOptions::new()
                        .left(0.try_into().unwrap())
                        .top(container_element.scroll_height().try_into().unwrap()),
                );
                on_submit_command.set("".to_owned());
                input_value.set("".to_owned());
            }

            if event.key() == "ArrowUp" {
                let commands_history = &*(history);
                let commands = commands_history
                    .into_iter()
                    .map(|history| &history.command)
                    .collect::<Vec<&String>>();

                event.prevent_default();
                let command_length = commands.len() as u32;
                if command_length == 0 as u32 {
                    return;
                }

                let index = *(last_command_index) + 1;
                if index <= command_length.clone().try_into().unwrap() {
                    last_command_index.set(index);
                    on_submit_command.set(commands[(&command_length - 1) as usize].to_owned())
                }
            }

            if event.key() == "ArrowDown" {
                let commands_history = &*(history);
                let commands = commands_history
                    .into_iter()
                    .map(|history| &history.command)
                    .collect::<Vec<&String>>();

                event.prevent_default();
                let command_length = commands.len();
                if command_length == 0 {
                    return;
                }
                let index = *(last_command_index) - 1;
                if index > 0 {
                    last_command_index.set(index);

                    on_submit_command.set(commands[&command_length - index as usize].to_owned())
                } else {
                    last_command_index.set(0);
                    on_submit_command.set("".to_owned());
                }
            }
        })
    };

    let onchange = {
        let input_value = input_value.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>();
            if let Some(input) = input {
                on_change_command.set(input.value());
                input_value.set(input.value())
            }
        })
    };

    // let onchange = {
    //     let input_node_ref = input_node_ref.clone();

    //     Callback::from(move |_| {
    //         let input = input_node_ref.cast::<HtmlInputElement>();

    //         if let Some(input) = input {
    //             input_value_handle.set(input.value());
    //         }
    //     })
    // };

    html! {

        <div class="flex flex-row space-x-2">
              <label htmlFor="prompt" class="flex-shrink">
                <Ps1 />
              </label>

              <input
                ref={input_ref}
                id="prompt"
                type="text"
                // need to have value soon.. .but it's pretty much hopeless
                value={(*input_value).clone()}
                {onchange}
                class="bg-light-background dark:bg-dark-background focus:outline-none flex-grow"
                autofocus={true}
                onkeydown={on_submit}
                autocomplete="off"
                //spell check... later

              />
            </div>
    }
}
