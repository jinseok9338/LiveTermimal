use yew::prelude::*;

use crate::utils::commands::commands_context_hook::COMMAND_LIST;

#[derive(Properties, Clone, PartialEq)]
pub struct HelpProps {
    pub args: Vec<String>,
}

#[function_component(HelpComponent)]
pub fn help(props: &HelpProps) -> Html {
    let args = props.args.clone();
    let mut result = html! {};
    // if args is 1 find the command in the list and display its description if args is 0 display all the commands
    let arg = args.get(1);
    if let Some(arg) = arg {
        // find the command from the COMMAND_LIST
        if let Some((command, description)) =
            COMMAND_LIST.iter().find(|(command, _)| command == arg)
        {
            // then display its description
            result = html!(
                <div class="whitespace-pre-wrap mb-2 leading-normal">
                    <div class="flex"><p class="text-description font-bold">{command}</p><p class="description">{" - "}</p><p class="description">{description}</p></div>
                </div>
            );
        }
    }

    if arg.is_none() {
        result = html! {
            <div class="whitespace-pre-wrap mb-2 leading-normal">
            { for COMMAND_LIST.iter().map(|(command, description)| html! {
                    <div class="flex"><p class="text-description font-bold">{command}</p><p class="description">{" - "}</p><p class="description">{description}</p></div>
            }) }
            </div>
        };
    }

    if args.len() > 2 {
        result = html!(
            <div class="whitespace-pre-wrap mb-2 leading-normal">
                <div class="flex"> <p class="description">{"You should only specify one command"}</p></div>
            </div>
        );
    }

    html! {
        <p>{ result }</p>
    }
}
