use crate::config::config::config::Config;
use std::fs;
use yew::prelude::*;

#[function_component(Ps1)]
pub fn ps_1() -> Html {
    let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");

    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");

    html! {
        <div>
            <span className="text-light-yellow dark:text-dark-yellow">
             {config.ps1_username}
           </span>
            <span className="text-light-gray dark:text-dark-gray">{"@"}</span>
            <span className="text-light-green dark:text-dark-green">
               {config.ps1_hostname}
             </span>
             <span className="text-light-gray dark:text-dark-gray">{":$ ~ "}</span>
        </div>
    }
}
