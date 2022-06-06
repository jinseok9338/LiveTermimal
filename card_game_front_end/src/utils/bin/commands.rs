use crate::config::config::config::Config;
use std::{fs, io::Error};
use web_sys::{window, Window};
use yew::{function_component, html, use_context, Children, ContextProvider, Properties};

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

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct CommandsContext {
    pub config: Config,
    pub window: Window,
}

impl CommandsContext {
    pub const COMMAND_LIST: Vec<&'static str> = vec![
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

    pub fn new(config: Config, window: Window) -> Self {
        Self { config, window }
    }

    pub fn help(self: &Self, _args: Vec<&str>) -> Result<String, Error> {
        let mut result_string = "".to_owned();
        for (i, command) in CommandsContext::COMMAND_LIST.into_iter().enumerate() {
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
    let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("JSON does not have correct format.");
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
