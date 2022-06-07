use crate::components::history::history_component::HistoryComponent;
use crate::components::history::hook::use_history;
use crate::components::history::input::Input;
use crate::utils::bin::commands::use_command;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainProps {
    pub input_ref: NodeRef,
}

#[function_component(Main)]
pub fn main(props: &MainProps) -> Html {
    let history_props = use_history();
    let cloned_history_props = history_props.clone();
    let history = &*(cloned_history_props.history.clone());
    let commands_context = use_command();
    let container_ref = use_node_ref();
    let input_ref = props.input_ref.clone();

    use_effect_with_deps(
        move |_| {
            {
                cloned_history_props.set_history(commands_context.banner().unwrap().to_owned());
            }
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
        [history.clone()],
    );

    html! {
        <>
      <div className="p-8 overflow-hidden h-full border-2 rounded border-light-yellow dark:border-dark-yellow">
        <div ref={&container_ref.clone()} className="overflow-y-auto h-full">
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
