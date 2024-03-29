use yew::prelude::*;

use crate::utils::commands::commands_context_hook::CONFIG;

#[function_component(Ps1)]
pub fn ps_1() -> Html {
    html! {
        <div>
            <span class="text-light-yellow dark:text-dark-yellow">{ CONFIG.ps1_username }</span>
            <span class="text-light-gray dark:text-dark-gray">{ "@" }</span>
            <span class="text-light-green dark:text-dark-green">{ CONFIG.ps1_hostname }</span>
            <span class="text-light-gray dark:text-dark-gray">{ ":$ ~ " }</span>
        </div>
    }
}
