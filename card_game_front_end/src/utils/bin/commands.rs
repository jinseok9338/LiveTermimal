use crate::config::config::config::Config;
use std::{fs, io::Error};
use web_sys::{window, Window};

struct Commands {
    config: Config,
    window: Window,
}

impl Commands {
    pub fn new() -> Self {
        let data = fs::read_to_string("./src/config/config.json").expect("Unable to read file");
        let config: Config =
            serde_json::from_str(&data).expect("JSON does not have correct format.");
        let window = window().unwrap();

        Self { config, window }
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
        Ok("this is temp ls".to_owned())
    }

    pub async fn cd(self: &Self, args: Vec<String>) -> Result<String, Error> {
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
