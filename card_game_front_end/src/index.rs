use std::sync::Arc;

use crate::components::history::history_component::HistoryComponent;

use crate::components::history::history_context_hook::HistoryContext;
use crate::components::history::history_function::set_history;
use crate::components::history::input::Input;

use crate::utils::commands::commands_context_hook::CommandsContext;
use crate::utils::commands::execute_command::banner;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IndexProps {
    pub input_ref: NodeRef,
}

#[function_component(Index)]
pub fn index(props: &IndexProps) -> Html {
    let history_context = use_context::<HistoryContext>().expect("no ctx found");
    let history_handler = history_context.history.clone();
    let history_handler_for_focus = history_context.history.clone();
    let history_for_focus = (*history_handler_for_focus).len();
    let command_handler = history_context.command;
    let container_ref = use_node_ref();
    let input_ref = props.input_ref.clone();

    //config context
    let command_context = use_context::<CommandsContext>().expect("no ctx found");
    let config = command_context.config;
    let command_list = command_context.command_list;
    // convert Vec<&str> to Vec<String>
    let command_list = command_list
        .iter()
        .map(|command| command.to_string())
        .collect::<Vec<String>>();

        let command_list = Arc::new(command_list);

    // this sets the history to the banner
    use_effect_with_deps(
        move |_| {
            set_history(
                &history_handler.clone(),
                &command_handler.clone(),
                banner(&config),
            );
            || {}
        },
        (),
    );

    use_effect_with_deps(
        move |_| {
            let input_element = input_ref.cast::<HtmlInputElement>().unwrap();
            input_element.scroll_into_view();
            input_element.focus().unwrap();
            //This is Clean Up
            || {}
        },
        [history_for_focus], // focus when the length of the histoey size changes
    );

    html! {
        <>
      <div class="p-8 overflow-hidden h-full border-2 rounded border-light-yellow dark:border-dark-yellow">
        <div ref={&container_ref.clone()} class="overflow-y-auto h-full">
            <HistoryComponent />
            <Input
            command_list = {&command_list}
            input_ref={&props.input_ref}
            container_ref={container_ref}
            />
        </div>
      </div>
    </>
    }
}
