use crate::config::config::config::Colors;
use crate::config::config::config::Config;
use crate::config::config::config::DonateURLs;
use crate::config::config::config::Social;
use crate::config::config::config::ThemeColors;
use gloo_console::log;
use std::{fs, io::Error};
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
        ];
        Self {
            config,
            window,
            command_list,
        }
    }

    pub fn help(self: &Self, _args: Vec<&str>) -> Result<String, Error> {
        let mut result_string = "".to_owned();
        for (i, command) in self.command_list.to_owned().into_iter().enumerate() {
            if i % 7 == 0 {
                result_string += &(command.to_owned() + "\n");
            } else {
                result_string += &(command.to_owned() + " ");
            }
        }

        Ok(format!(
            "Welcome! Here are all the available commands:
        \n{result_string}\n
        [tab]: trigger completion.
        [ctrl+l]/clear: clear terminal.\n
        Type 'sumfetch' to display summary.",
            result_string = result_string
        ))
    }

    //Redirection to repo
    pub fn repo(self: &Self, _args: Vec<&str>) -> Result<String, Error> {
        self.window
            .open_with_url(self.config.repo.as_ref())
            .unwrap();

        Ok("Opening Github repository...".to_owned())
    }

    //About
    pub fn about(self: &Self, _args: Vec<&str>) -> Result<String, Error> {
        Ok(format!(
            r#"Hi, I am {name}.
        Welcome to my website!
        More about me:
        'sumfetch' - short summary.
        'resume' - my latest resume.
        'readme' - my github readme.`
    "#,
            name = self.config.name
        )
        .to_owned())
    }

    pub fn resume(self: &Self, _args: Vec<&str>) -> Result<String, Error> {
        self.window
            .open_with_url(self.config.resume_url.as_ref())
            .unwrap();

        Ok("Opening resume".to_owned())
    }

    pub fn donate(self: &Self, _args: Vec<&str>) -> Result<String, Error> {
        Ok(r#"
        thank you for your interest.
        here are the ways you can support my work:
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.paypal}" target="_blank">paypal</a></u>
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.patreon}" target="_blank">patreon</a></u>
        "#.to_owned())
    }

    pub fn google(self: &Self, args: Vec<&str>) -> Result<String, Error> {
        let query = args.join(" ");
        self.window
            .open_with_url(format!("https://google.com/search?q=${query}", query = query).as_ref())
            .unwrap();
        Ok(format!("Searching google for {query}...", query = query))
    }

    pub fn duckduckgo(self: &Self, args: Vec<&str>) -> Result<String, Error> {
        let query = args.join(" ");
        self.window
            .open_with_url(format!("https://duckduckgo.com/?q=${query}", query = query).as_ref())
            .unwrap();
        Ok(format!(
            "Searching duckduckgo for {query}...",
            query = query
        ))
    }

    pub fn bing(self: &Self, args: Vec<&str>) -> Result<String, Error> {
        let query = args.join(" ");
        self.window
            .open_with_url(format!("https://bing.com/search?q=${query}", query = query).as_ref())
            .unwrap();
        Ok(format!(
            "Searching bing for {query}... but seriously Really?",
            query = query
        ))
    }

    pub fn reddit(self: &Self, args: Vec<&str>) -> Result<String, Error> {
        let query = args.join(" ");
        self.window
            .open_with_url(format!("https://reddit.com/search/?q=${query}", query = query).as_ref())
            .unwrap();
        Ok(format!("Searching reddit for {query}...", query = query))
    }

    //Typical linux Commands
    pub fn echo(self: &Self, args: Vec<&str>) -> Result<String, Error> {
        let query = args.join(" ");
        Ok(query)
    }

    pub fn whoami(self: &Self, _args: Vec<&str>) -> Result<String, Error> {
        Ok(self.config.ps1_username.to_owned())
    }

    pub fn ls(self: &Self, _args: Vec<&str>) -> Result<String, Error> {
        todo! {}
        Ok("this is temp ls".to_owned())
    }

    pub fn cd(self: &Self, _args: Vec<&str>) -> Result<String, Error> {
        todo! {}
        Ok("this is temp cd".to_owned())
    }

    pub fn banner(self: &Self) -> Result<String, Error> {
        Ok(r#"
        █████        ███                       ███████████
        ░░███        ░░░                       ░█░░░███░░░█
        ░███        ████  █████ █████  ██████ ░   ░███  ░   ██████  ████████  █████████████
        ░███       ░░███ ░░███ ░░███  ███░░███    ░███     ███░░███░░███░░███░░███░░███░░███
        ░███        ░███  ░███  ░███ ░███████     ░███    ░███████  ░███ ░░░  ░███ ░███ ░███
        ░███      █ ░███  ░░███ ███  ░███░░░      ░███    ░███░░░   ░███      ░███ ░███ ░███
        ███████████ █████  ░░█████   ░░██████     █████   ░░██████  █████     █████░███ █████
        ░░░░░░░░░░░ ░░░░░    ░░░░░     ░░░░░░     ░░░░░     ░░░░░░  ░░░░░     ░░░░░ ░░░ ░░░░░
        Type 'help' to see the list of available commands.
        Type 'sumfetch' to display summary.
        Type 'repo' or click <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.repo}" target="_blank">here</a></u> for the Github repository.
        "#.to_owned())
    }

    pub async fn execute_command(
        self: &Self,
        command: String,
        args: Vec<&str>,
    ) -> Result<String, Error> {
        match command.as_str() {
            "help" => Ok(self.help(args).unwrap()),
            "banner" => Ok(self.banner().unwrap()),
            "about" => Ok(self.about(args).unwrap()),
            "bing" => Ok(self.bing(args).unwrap()),
            "repo" => Ok(self.repo(args).unwrap()),
            "resume" => Ok(self.resume(args).unwrap()),
            "donate" => Ok(self.donate(args).unwrap()),
            "google" => Ok(self.google(args).unwrap()),
            "duckduckgo" => Ok(self.duckduckgo(args).unwrap()),
            "reddit" => Ok(self.reddit(args).unwrap()),
            "whoami" => Ok(self.whoami(args).unwrap()),
            "ls" => Ok(self.ls(args).unwrap()),
            "cd" => Ok(self.cd(args).unwrap()),
            &_ => Ok("Unvalid Command...  type 'help' to get started".to_owned()),
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct CommandProviderProps {
    pub children: Children,
}

#[function_component(CommandContextProvider)]
pub fn history_provider(props: &CommandProviderProps) -> Html {
    let config: Config = Config::new({
        &Config {
            readme_url: "https://raw.githubusercontent.com/cveinnt/cveinnt/master/README.md"
                .to_owned(),
            title: "LiveTerm".to_owned(),
            name: "John Doe".to_owned(),
            ascii: "liveterm".to_owned(),
            social: Social {
                github: "github".to_owned(),
                linkedin: "linkedin".to_owned(),
            },
            email: "contact@email.com".to_owned(),
            ps1_hostname: "liveterm".to_owned(),
            ps1_username: "visitor".to_owned(),
            repo: "https://github.com/Cveinnt/LiveTerm".to_owned(),
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
