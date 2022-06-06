use crate::config::config::config::Config;
use std::{io::Error, fs};
use web_sys::{Window, window};
use yew::{function_component, html, use_context, use_state, Children, Properties ContextProvider};

pub enum CommandsList {
    Help,
    Repo,
    About,
    Resume,
    Donate,
    Google,
    Duckduckgo,
    Bing,
    Reddit,
    Echo,
    Whoami,
    Ls,
    Cd,
    Banner,
}

fn commands_in_text(command: CommandsList) -> Result<String, Error> {
    match command {
        CommandsList::About => Ok("about".to_owned()),
        CommandsList::Banner => Ok("banner".to_owned()),
        CommandsList::Bing => Ok("bing".to_owned()),
        CommandsList::Help => Ok("help".to_owned()),
        CommandsList::Repo => Ok("repo".to_owned()),
        CommandsList::Resume => Ok("resume".to_owned()),
        CommandsList::Donate => Ok("donate".to_owned()),
        CommandsList::Google => Ok("google".to_owned()),
        CommandsList::Duckduckgo => Ok("duckduckgo".to_owned()),
        CommandsList::Reddit => Ok("reddit".to_owned()),
        CommandsList::Echo => Ok("echo".to_owned()),
        CommandsList::Whoami => Ok("whoami".to_owned()),
        CommandsList::Ls => Ok("ls".to_owned()),
        CommandsList::Cd => Ok("cd".to_owned()),
    }
}

#[derive(Debug, PartialEq, Properties,Clone)]
pub struct CommandsContext {
    config: Config,
    window: Window,
}

impl CommandsContext {
    pub const COMMAND_LIST: Vec<String> = vec![
        "about".to_owned(),
        "banner".to_owned(),
        "bing".to_owned(),
        "help".to_owned(),
        "repo".to_owned(),
        "resume".to_owned(),
        "donate".to_owned(),
        "google".to_owned(),
        "duckduckgo".to_owned(),
        "reddit".to_owned(),
        "echo".to_owned(),
        "whoami".to_owned(),
        "ls".to_owned(),
        "cd".to_owned(),
    ];

    pub fn new (config: Config, window:Window) -> Self {
        Self {
            config,
            window,
        }
    }

    pub fn help(self: &Self, args: Vec<String>) -> Result<String, Error> {
        todo! {}
    }

    //Redirection to repo
    pub async fn repo(self: &Self, args: Vec<String>) -> Result<String, Error> {
        self.window
            .open_with_url(self.config.repo.as_ref())
            .unwrap();

        Ok("Opening Github repository...".to_owned())
    }

    //About
    pub async fn about(self: &Self, args: Vec<String>) -> Result<String, Error> {
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

    pub async fn resume(self: &Self, args: Vec<String>) -> Result<String, Error> {
        self.window
            .open_with_url(self.config.resume_url.as_ref())
            .unwrap();

        Ok("Opening resume".to_owned())
    }

    pub async fn donate(self: &Self, args: Vec<String>) -> Result<String, Error> {
        Ok(r#"
        thank you for your interest.
        here are the ways you can support my work:
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.paypal}" target="_blank">paypal</a></u>
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.patreon}" target="_blank">patreon</a></u>
        "#.to_owned())
    }

    pub async fn google(self: &Self, args: Vec<String>) -> Result<String, Error> {
        let query = args.join(" ");
        self.window
            .open_with_url(format!("https://google.com/search?q=${query}", query = query).as_ref())
            .unwrap();
        Ok(format!("Searching google for {query}...", query = query))
    }

    pub async fn duckduckgo(self: &Self, args: Vec<String>) -> Result<String, Error> {
        let query = args.join(" ");
        self.window
            .open_with_url(format!("https://duckduckgo.com/?q=${query}", query = query).as_ref())
            .unwrap();
        Ok(format!(
            "Searching duckduckgo for {query}...",
            query = query
        ))
    }

    pub async fn bing(self: &Self, args: Vec<String>) -> Result<String, Error> {
        let query = args.join(" ");
        self.window
            .open_with_url(format!("https://bing.com/search?q=${query}", query = query).as_ref())
            .unwrap();
        Ok(format!(
            "Searching bing for {query}... but seriously Really?",
            query = query
        ))
    }

    pub async fn reddit(self: &Self, args: Vec<String>) -> Result<String, Error> {
        let query = args.join(" ");
        self.window
            .open_with_url(format!("https://reddit.com/search/?q=${query}", query = query).as_ref())
            .unwrap();
        Ok(format!("Searching reddit for {query}...", query = query))
    }

    //Typical linux Commands
    pub async fn echo(self: &Self, args: Vec<String>) -> Result<String, Error> {
        let query = args.join(" ");
        Ok(query)
    }

    pub async fn whoami(self: &Self, args: Vec<String>) -> Result<String, Error> {
        Ok(self.config.ps1_username.to_owned())
    }

    pub async fn ls(self: &Self, args: Vec<String>) -> Result<String, Error> {
        todo! {}
        Ok("this is temp ls".to_owned())
    }

    pub async fn cd(self: &Self, args: Vec<String>) -> Result<String, Error> {
        todo! {}
        Ok("this is temp cd".to_owned())
    }

    pub async fn banner(self: &Self, args: Vec<String>) -> Result<String, Error> {
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
}

#[derive(Debug, PartialEq, Properties)]
pub struct CommandProviderProps {
    pub children: Children,
}

#[function_component(CommandContextProvider)]
pub fn history_provider(props: &CommandProviderProps) -> Html {
    let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");
    let window = window().unwrap();

    let history_ctx = CommandsContext::new(config,window);

    html! {
        <ContextProvider<CommandsContext> context={history_ctx}>
            {props.children.clone()}
        </ContextProvider<CommandsContext>>
    }
}

pub fn use_command() -> CommandsContext {
    use_context::<CommandsContext>().expect("no ctx found")
}
