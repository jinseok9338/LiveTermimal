use crate::utils::commands::commands::use_command;
use yew::prelude::*;

#[function_component(Ps1)]
pub fn ps_1() -> Html {
    let command_context = use_command();
    let config = command_context.config;
    html! {
        <div>
            <span class="text-light-yellow dark:text-dark-yellow">
             {config.ps1_username}
           </span>
            <span class="text-light-gray dark:text-dark-gray">{"@"}</span>
            <span class="text-light-green dark:text-dark-green">
               {config.ps1_hostname}
             </span>
             <span class="text-light-gray dark:text-dark-gray">{":$ ~ "}</span>
        </div>
    }
}
