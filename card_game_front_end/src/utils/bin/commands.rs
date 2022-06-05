// // List of commands that do not require API calls

// import * as bin from './index';
// import config from '../../../config.json';

// // Help
// export const help = async (args: string[]): Promise<string> => {
//   const commands = Object.keys(bin).sort().join(', ');
//   var c = '';
//   for (let i = 1; i <= Object.keys(bin).sort().length; i++) {
//     if (i % 7 === 0) {
//       c += Object.keys(bin).sort()[i - 1] + '\n';
//     } else {
//       c += Object.keys(bin).sort()[i - 1] + ' ';
//     }
//   }
//   return `Welcome! Here are all the available commands:
// \n${c}\n
// [tab]: trigger completion.
// [ctrl+l]/clear: clear terminal.\n
// Type 'sumfetch' to display summary.
// `;
// };

// // Redirection
// export const repo = async (args: string[]): Promise<string> => {
//   window.open(`${config.repo}`);
//   return 'Opening Github repository...';
// };

// // About
// export const about = async (args: string[]): Promise<string> => {
//   return `Hi, I am ${config.name}.
// Welcome to my website!
// More about me:
// 'sumfetch' - short summary.
// 'resume' - my latest resume.
// 'readme' - my github readme.`;
// };

// export const resume = async (args: string[]): Promise<string> => {
//   window.open(`${config.resume_url}`);
//   return 'Opening resume...';
// };

// // Donate
// export const donate = async (args: string[]): Promise<string> => {
//   return `thank you for your interest.
// here are the ways you can support my work:
// - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.paypal}" target="_blank">paypal</a></u>
// - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.patreon}" target="_blank">patreon</a></u>
// `;
// };

// // Contact
// export const email = async (args: string[]): Promise<string> => {
//   window.open(`mailto:${config.email}`);
//   return `Opening mailto:${config.email}...`;
// };

// export const github = async (args: string[]): Promise<string> => {
//   window.open(`https://github.com/${config.social.github}/`);

//   return 'Opening github...';
// };

// export const linkedin = async (args: string[]): Promise<string> => {
//   window.open(`https://www.linkedin.com/in/${config.social.linkedin}/`);

//   return 'Opening linkedin...';
// };

// // Search
// export const google = async (args: string[]): Promise<string> => {
//   window.open(`https://google.com/search?q=${args.join(' ')}`);
//   return `Searching google for ${args.join(' ')}...`;
// };

// export const duckduckgo = async (args: string[]): Promise<string> => {
//   window.open(`https://duckduckgo.com/?q=${args.join(' ')}`);
//   return `Searching duckduckgo for ${args.join(' ')}...`;
// };

// export const bing = async (args: string[]): Promise<string> => {
//   window.open(`https://bing.com/search?q=${args.join(' ')}`);
//   return `Wow, really? You are using bing for ${args.join(' ')}?`;
// };

// export const reddit = async (args: string[]): Promise<string> => {
//   window.open(`https://www.reddit.com/search/?q=${args.join(' ')}`);
//   return `Searching reddit for ${args.join(' ')}...`;
// };

// // Typical linux commands
// export const echo = async (args: string[]): Promise<string> => {
//   return args.join(' ');
// };

// export const whoami = async (args: string[]): Promise<string> => {
//   return `${config.ps1_username}`;
// };

// export const ls = async (args: string[]): Promise<string> => {
//   return `a
// bunch
// of
// fake
// directories`;
// };

// export const cd = async (args: string[]): Promise<string> => {
//   return `unfortunately, i cannot afford more directories.
// if you want to help, you can type 'donate'.`;
// };

// export const date = async (args: string[]): Promise<string> => {
//   return new Date().toString();
// };

// export const vi = async (args: string[]): Promise<string> => {
//   return `woah, you still use 'vi'? just try 'vim'.`;
// };

// export const vim = async (args: string[]): Promise<string> => {
//   return `'vim' is so outdated. how about 'nvim'?`;
// };

// export const nvim = async (args: string[]): Promise<string> => {
//   return `'nvim'? too fancy. why not 'emacs'?`;
// };

// export const emacs = async (args?: string[]): Promise<string> => {
//   return `you know what? just use vscode.`;
// };

// export const sudo = async (args?: string[]): Promise<string> => {
//   window.open('https://www.youtube.com/watch?v=dQw4w9WgXcQ', '_blank'); // ...I'm sorry
//   return `Permission denied: with little power comes... no responsibility? `;
// };

// // Banner
// export const banner = (args?: string[]): string => {
//   return `
// █████        ███                       ███████████
// ░░███        ░░░                       ░█░░░███░░░█
//  ░███        ████  █████ █████  ██████ ░   ░███  ░   ██████  ████████  █████████████
//  ░███       ░░███ ░░███ ░░███  ███░░███    ░███     ███░░███░░███░░███░░███░░███░░███
//  ░███        ░███  ░███  ░███ ░███████     ░███    ░███████  ░███ ░░░  ░███ ░███ ░███
//  ░███      █ ░███  ░░███ ███  ░███░░░      ░███    ░███░░░   ░███      ░███ ░███ ░███
//  ███████████ █████  ░░█████   ░░██████     █████   ░░██████  █████     █████░███ █████
// ░░░░░░░░░░░ ░░░░░    ░░░░░     ░░░░░░     ░░░░░     ░░░░░░  ░░░░░     ░░░░░ ░░░ ░░░░░
// Type 'help' to see the list of available commands.
// Type 'sumfetch' to display summary.
// Type 'repo' or click <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.repo}" target="_blank">here</a></u> for the Github repository.
// `;
// };

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
