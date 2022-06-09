use std::io::Error;

use web_sys::Window;

use crate::config::config::config::Config;

pub fn help(_args: Vec<&str>, command_list: Vec<&'static str>) -> Result<String, Error> {
    let mut result_string = "".to_owned();
    for (i, command) in command_list.to_owned().into_iter().enumerate() {
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
pub fn repo(_args: Vec<&str>, window: Window, config: Config) -> Result<String, Error> {
    window.open_with_url(config.repo.as_ref()).unwrap();

    Ok("Opening Github repository...".to_owned())
}

//About
pub fn about(_args: Vec<&str>, config: Config) -> Result<String, Error> {
    Ok(format!(
        r#"Hi, I am {name}.
        Welcome to my website!
        More about me:
        'sumfetch' - short summary.
        'resume' - my latest resume.
        'readme' - my github readme.`
    "#,
        name = config.name
    )
    .to_owned())
}

pub fn resume(_args: Vec<&str>, window: Window, config: Config) -> Result<String, Error> {
    window.open_with_url(config.resume_url.as_ref()).unwrap();

    Ok("Opening resume".to_owned())
}

pub fn donate(_args: Vec<&str>) -> Result<String, Error> {
    Ok(r#"
        thank you for your interest.
        here are the ways you can support my work:
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.paypal}" target="_blank">paypal</a></u>
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.patreon}" target="_blank">patreon</a></u>
        "#.to_owned())
}

pub fn google(args: Vec<&str>, window: Window) -> Result<String, Error> {
    let query = args.join(" ");
    window
        .open_with_url(format!("https://google.com/search?q=${query}", query = query).as_ref())
        .unwrap();
    Ok(format!("Searching google for {query}...", query = query))
}

pub fn duckduckgo(args: Vec<&str>, window: Window) -> Result<String, Error> {
    let query = args.join(" ");
    window
        .open_with_url(format!("https://duckduckgo.com/?q=${query}", query = query).as_ref())
        .unwrap();
    Ok(format!(
        "Searching duckduckgo for {query}...",
        query = query
    ))
}

pub fn bing(args: Vec<&str>, window: Window) -> Result<String, Error> {
    let query = args.join(" ");
    window
        .open_with_url(format!("https://bing.com/search?q=${query}", query = query).as_ref())
        .unwrap();
    Ok(format!(
        "Searching bing for {query}... but seriously Really?",
        query = query
    ))
}

pub fn reddit(args: Vec<&str>, window: Window) -> Result<String, Error> {
    let query = args.join(" ");
    window
        .open_with_url(format!("https://reddit.com/search/?q=${query}", query = query).as_ref())
        .unwrap();
    Ok(format!("Searching reddit for {query}...", query = query))
}

//Typical linux Commands
pub fn echo(args: Vec<&str>) -> Result<String, Error> {
    let query = args.join(" ");
    Ok(query)
}

pub fn whoami(_args: Vec<&str>, config: Config) -> Result<String, Error> {
    Ok(config.ps1_username.to_owned())
}

pub fn ls(_args: Vec<&str>) -> Result<String, Error> {
    todo! {}
    Ok("this is temp ls".to_owned())
}

pub fn cd(_args: Vec<&str>) -> Result<String, Error> {
    todo! {}
    Ok("this is temp cd".to_owned())
}

pub fn banner() -> Result<String, Error> {
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

pub fn execute_command(
    command: String,
    args: Vec<&str>,
    window: Window,
    config: Config,
    command_list: Vec<&'static str>,
) -> Result<String, Error> {
    match command.as_str() {
        "help" => Ok(help(args, command_list).unwrap()),
        "banner" => Ok(banner().unwrap()),
        "about" => Ok(about(args, config).unwrap()),
        "bing" => Ok(bing(args, window).unwrap()),
        "repo" => Ok(repo(args, window, config).unwrap()),
        "resume" => Ok(resume(args, window, config).unwrap()),
        "donate" => Ok(donate(args).unwrap()),
        "google" => Ok(google(args, window).unwrap()),
        "duckduckgo" => Ok(duckduckgo(args, window).unwrap()),
        "reddit" => Ok(reddit(args, window).unwrap()),
        "whoami" => Ok(whoami(args, config).unwrap()),
        "ls" => Ok(ls(args).unwrap()),
        "cd" => Ok(cd(args).unwrap()),
        &_ => Ok("Unvalid Command...  type 'help' to get started".to_owned()),
    }
}
