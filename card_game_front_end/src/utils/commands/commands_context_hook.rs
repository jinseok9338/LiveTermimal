use crate::config::config::config::Colors;
use crate::config::config::config::Config;
use crate::config::config::config::DonateURLs;
use crate::config::config::config::Social;
use crate::config::config::config::ThemeColors;

use web_sys::{window, Window};
use yew::{function_component, html, use_context, Children, ContextProvider, Properties};
#[derive(Debug, PartialEq, Properties, Clone)]
pub struct CommandsContext {
    pub config: Config,
    pub window: Window,
    pub command_list: Vec<&'static str>,
}

impl CommandsContext {
    pub fn new(config: Config, window: Window) -> Self {
        let command_list: Vec<&'static str> = vec![
            "about",
            "banner",
            "bing",
            "help",
            "repo",
            "resume",
            "donate",
            "google",
            "duckduckgo",
            "reddit",
            "echo",
            "whoami",
            "ls",
            "cd",
            "sumfetch",
            "theme",
        ];
        Self {
            config,
            window,
            command_list,
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct CommandProviderProps {
    pub children: Children,
}

#[function_component(CommandContextProvider)]
pub fn command_provider(props: &CommandProviderProps) -> Html {
    let config: Config = Config::new({
        &Config {
            readme_url: "https://raw.githubusercontent.com/jinseok9338/CardGame/master/README.md"
                .to_owned(),
            title: "JTerm".to_owned(),
            name: "Jason Jin (Jinseok)".to_owned(),
            ascii: "jason".to_owned(),
            social: Social {
                github: "jinseok9338".to_owned(),
                linkedin: "linkedin".to_owned(),
            },
            email: "jinseok9338@email.com".to_owned(),
            ps1_hostname: "Jason Jin".to_owned(),
            ps1_username: "visitor".to_owned(),
            repo: "https://github.com/jinseok9338/CardGame".to_owned(),
            resume_url: "https://upload.wikimedia.org/wikipedia/commons/c/cc/Resume.pdf".to_owned(),
            donate_urls: DonateURLs {
                paypal: "https://paypal.me/cveinnt".to_owned(),
                patreon: "https://patreon.com/cveinnt".to_owned(),
            },
            colors: ThemeColors {
                light: Colors {
                    background: "#FBF1C9".to_owned(),
                    foreground: "#3C3836".to_owned(),
                    yellow: "#D79921".to_owned(),
                    green: "#98971A".to_owned(),
                    gray: "#7C6F64".to_owned(),
                    blue: "#458588".to_owned(),
                    red: "#CA2124".to_owned(),
                },
                dark: Colors {
                    background: "#2E3440".to_owned(),
                    foreground: "#E5E9F0".to_owned(),
                    yellow: "#5E81AC".to_owned(),
                    green: "#A3BE8C".to_owned(),
                    gray: "#88C0D0".to_owned(),
                    blue: "#EBCB8B".to_owned(),
                    red: "#BF616A".to_owned(),
                },
            },
        }
    });
    let window = window().unwrap();

    let history_ctx = CommandsContext::new(config, window);

    html! {
        <ContextProvider<CommandsContext> context={history_ctx}>
            {props.children.clone()}
        </ContextProvider<CommandsContext>>
    }
}

pub fn use_command() -> CommandsContext {
    use_context::<CommandsContext>().expect("no ctx found")
}
