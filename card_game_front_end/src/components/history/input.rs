use web_sys::{HtmlElement, HtmlInputElement, ScrollToOptions};
use yew::prelude::*;

use crate::components::history::hook::use_history;
use crate::components::ps_1::Ps1;
use crate::utils::shell::shell;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub input_ref: NodeRef,
    pub container_ref: NodeRef,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let input_ref = &props.input_ref;
    let history = use_history();
    let cloned_history = history.clone();
    let command = &*cloned_history.command.clone();
    let container_element = props.container_ref.cast::<HtmlElement>().unwrap();

    let on_submit = {
        let cloned_history = history.clone();

        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "c".to_owned() && event.ctrl_key() {
                event.prevent_default();
                cloned_history.command.set("".to_owned());
                cloned_history.set_history("".to_owned());
                cloned_history.last_command_index.set(0);
            }

            if event.key() == "l".to_owned() && event.ctrl_key() {
                event.prevent_default();
                cloned_history.clear_history();
            }

            if event.key() == "Tab".to_owned() {
                event.prevent_default();
                todo! {} //handle_tab_completion(command, cloned_history.command.set);
            }

            if event.key() == "Enter".to_owned() || event.code() == "13".to_owned() {
                event.prevent_default();
                cloned_history.last_command_index.set(0);
                shell();
                container_element.scroll_to_with_scroll_to_options(
                    &ScrollToOptions::new()
                        .left(0.try_into().unwrap())
                        .top(container_element.scroll_height().try_into().unwrap()),
                )
            }

            if event.key() == "ArrowUp" {
                let commands_history = &*(cloned_history.history);
                let commands = commands_history
                    .into_iter()
                    .map(|history| &history.command)
                    .collect::<Vec<&String>>();

                event.prevent_default();
                let command_length = commands.len() as u32;
                if command_length == 0 as u32 {
                    return;
                }

                let index = *(cloned_history.last_command_index) + 1;
                if index <= command_length.clone().try_into().unwrap() {
                    cloned_history.last_command_index.set(index);
                    cloned_history
                        .command
                        .set(commands[(&command_length - 1) as usize].to_owned())
                }
            }

            if event.key() == "ArrowDown" {
                let commands_history = &*(cloned_history.history);
                let commands = commands_history
                    .into_iter()
                    .map(|history| &history.command)
                    .collect::<Vec<&String>>();

                event.prevent_default();
                let command_length = commands.len();
                if command_length == 0 {
                    return;
                }
                let index = *(cloned_history.last_command_index) - 1;
                if index > 0 {
                    cloned_history.last_command_index.set(index);
                    cloned_history
                        .command
                        .set(commands[&command_length - index].to_owned())
                } else {
                    cloned_history.last_command_index.set(0);
                    cloned_history.command.set("".to_owned());
                }
            }
        })
    };

    let onchange = Callback::from(move |input_event: Event| {
        let cloned_history = history.clone();
        let value = input_event
            .target_unchecked_into::<HtmlInputElement>()
            .value();
        cloned_history.command.set(value)
    });

    html! {
        <div class="flex flex-row space-x-2">
              <label htmlFor="prompt" class="flex-shrink">
                <Ps1 />
              </label>

              <input
                ref={input_ref}
                id="prompt"
                type="text"
                value={command.to_owned()}
                onchange={onchange}
                class={"bg-light-background dark:bg-dark-background focus:outline-none flex-grow ${
                               commandExists(command) || command === ''
                                 ? 'text-dark-green'
                                 : 'text-dark-red'
                             }"}
                autofocus={true}
                onkeydown={on_submit}
                autocomplete="off"
                //spell check... later
              />
            </div>
    }
}
