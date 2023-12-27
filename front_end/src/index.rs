use crate::components::history::history_component::HistoryComponent;
use crate::components::history::history_context_hook::HistoryContext;
use crate::components::history::history_function::set_history;
use crate::components::history::input::Input;

use crate::utils::commands::commands_context_hook::CONFIG;
use crate::utils::commands::execute_command::welcome_string;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IndexProps {
    pub input_ref: NodeRef,
}

#[function_component(Index)]
pub fn index(props: &IndexProps) -> Html {
    let history_context = use_context::<HistoryContext>().unwrap();
    let history_handler = history_context.history.clone();
    let history_handler_for_focus = history_context.history.clone();
    let history_for_focus = (*history_handler_for_focus).len();
    let command_handler = history_context.command.clone();
    let is_program_running = history_context.is_running.clone();

    let container_ref = use_node_ref();
    let input_ref = props.input_ref.clone();

    //config context

    use_effect_with((), move |_| {
        let (output_component, operation) = welcome_string(&CONFIG).unwrap();

        let command = &*command_handler;
        set_history(
            history_handler.clone(),
            command.to_owned(),
            output_component,
            operation,
        );

        || {}
    });

    use_effect_with([history_for_focus], move |_| {
        let input_element = input_ref.cast::<HtmlInputElement>();
        match input_element {
            Some(element) => {
                element.scroll_into_view();
                element.focus().unwrap();
            }
            None => {}
        }
        || {}
    });

    html! {
        <>
            <div
                class="p-8 overflow-hidden h-full border-2 rounded border-light-yellow dark:border-dark-yellow"
            >
                 <div ref={&container_ref.clone()} class="overflow-y-auto h-full">
                    <HistoryComponent />
                    {if !*is_program_running {
                        html! {<Input input_ref={&props.input_ref} container_ref={container_ref} />}
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </>
    }
}
