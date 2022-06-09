use crate::components::history::history_component::HistoryComponent;
use crate::components::history::history_context_hook::use_history;
use crate::components::history::history_function::set_history;
use crate::components::history::input::Input;

use crate::utils::commands::commands_context_hook::use_command;
use crate::utils::commands::execute_command::banner;
use gloo_console::log;
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
    let command_handler = history_context.command.clone();

    let commands_context = use_command();

    let container_ref = use_node_ref();
    let input_ref = props.input_ref.clone();

    // useCallback for the banner and it loads the commands...

    use_effect_with_deps(
        move |_| {
            set_history(
                history_handler,
                command_handler,
                banner().unwrap().to_owned(),
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
        [*history_handler.clone()],
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
