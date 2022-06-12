use crate::components::history::history_component::HistoryComponent;
use crate::components::history::history_context_hook::use_history;
use crate::components::history::history_function::set_history;
use crate::components::history::input::Input;

use crate::utils::commands::commands_context_hook::use_command;
use crate::utils::commands::execute_command::banner;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IndexProps {
    pub input_ref: NodeRef,
}

#[function_component(Index)]
pub fn index(props: &IndexProps) -> Html {
    let history_context = use_history();
    let history_handler = history_context.history.clone();
    let history_handler_for_focus = history_context.history.clone();
    let history_for_focus = (*history_handler_for_focus).len();
    let command_handler = history_context.command.clone();

    let container_ref = use_node_ref();
    let input_ref = props.input_ref.clone();

    //config context
    let command_context = use_command();
    let config = command_context.config.clone();

    use_effect_with_deps(
        move |_| {
            set_history(
                history_handler.clone(),
                command_handler.clone(),
                banner(config).unwrap().to_owned(),
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
            input_ref={&props.input_ref}
            container_ref={container_ref}
            />
        </div>
      </div>
    </>
    }
}
