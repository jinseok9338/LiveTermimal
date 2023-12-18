use std::io::Error;

use web_sys::Window;

use crate::config::config::config::Config;
use lazy_static::lazy_static;

use regex::Regex;

use super::{
    api_commands::{projects, quote, read_me, weather},
    sumfetch::sumfetch,
};

pub fn help(_args: Vec<&str>, command_list: Vec<&'static str>) -> Result<String, Error> {
    let mut result_string = String::new();

    for (i, command) in command_list.iter().enumerate() {
        if i % 7 == 0 && i != 0 {
            result_string.push_str("<br>");
        }

        result_string.push_str(&format!("<span>{}</span>", command));
        if i % 7 != 6 {
            result_string.push(' ');
        }
    }

    Ok(format!(
        "Welcome! Here are all the available commands:<br>{}
        <br>[tab]: trigger completion.<br>
        [ctrl+l]/clear: clear terminal.<br>
        <span class='font-bold'>Type 'sumfetch' to display summary.<span>",
        result_string
    ))
}

//Redirection to repo
pub fn repo(
    _args: Vec<&str>,
    window: Window,
    config: &'static Config<'static>,
) -> Result<String, Error> {
    window.open_with_url(config.repo.as_ref()).unwrap();

    Ok("Opening Github repository...".to_owned())
}

//About
pub fn about(_args: Vec<&str>, config: &'static Config<'static>) -> Result<String, Error> {
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

pub fn resume(
    _args: Vec<&str>,
    window: Window,
    config: &'static Config<'static>,
) -> Result<String, Error> {
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
    let query = args[1..].join(" ");
    return match query.eq("") {
        true => Ok(r#"
        You should provide query
        like this: google facetime 
        "#
        .to_owned()),
        _ => {
            window
                .open_with_url(
                    format!("https://google.com/search?q={query}", query = query).as_ref(),
                )
                .unwrap();
            Ok(format!("Searching google for {query}...", query = query).to_owned())
        }
    };
}

pub fn duckduckgo(args: Vec<&str>, window: Window) -> Result<String, Error> {
    let query = args[1..].join(" ");
    return match query.eq("") {
        true => Ok(r#"
        You should provide query
        like this: duckduckgo facetime 
        "#
        .to_owned()),
        _ => {
            window
                .open_with_url(format!("https://duckduckgo.com/?q={query}", query = query).as_ref())
                .unwrap();
            Ok(format!("Searching duckduckgo for {query}...", query = query).to_owned())
        }
    };
}

pub fn bing(args: Vec<&str>, window: Window) -> Result<String, Error> {
    let query = args[1..].join(" ");
    return match query.eq("") {
        true => Ok(r#"
        You should provide query
        like this: bing facetime 
        "#
        .to_owned()),
        _ => {
            window
                .open_with_url(format!("https://bing.com/search?q={query}", query = query).as_ref())
                .unwrap();
            Ok(format!("Searching bing for {query}...", query = query).to_owned())
        }
    };
}

pub fn reddit(args: Vec<&str>, window: Window) -> Result<String, Error> {
    let query = args[1..].join(" ");
    return match query.eq("") {
        true => Ok(r#"
        You should provide query
        like this: reddit facetime 
        "#
        .to_owned()),
        _ => {
            window
                .open_with_url(
                    format!("https://reddit.com/search/?q={query}", query = query).as_ref(),
                )
                .unwrap();
            Ok(format!("Searching reddit for {query}...", query = query).to_owned())
        }
    };
}

fn some_helper_function(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<[\s\S]*>").unwrap();
    }
    RE.is_match(text)
}

//Typical linux Commands
pub fn echo(args: Vec<&str>) -> Result<String, Error> {
    let query = args[1..].join(" ");
    match some_helper_function(&query) {
        true => Ok("You cheeky bastard... You are not allowed to type that".to_string()),
        false => Ok(query),
    }
}

pub fn whoami(_args: Vec<&str>, config: &'static Config<'static>) -> Result<String, Error> {
    Ok(config.ps1_username.to_owned())
}

pub fn ls(_args: Vec<&str>) -> Result<String, Error> {
    Ok(r#"
    I
    don't 
    know 
    how 
    to add file system in 
    webAssemly"#
        .to_owned())
}

pub fn cd(_args: Vec<&str>) -> Result<String, Error> {
    Ok(r#"
    I
    don't 
    know 
    how 
    to add file system in 
    webAssemly"#
        .to_owned())
}

pub fn banner(config: &'static Config<'static>) -> Result<String, Error> {
    Ok(format!(r#"
    <span class="font-bold text-3xl">Welcome To</span>
    </pre>
    <pre class="animate-twinkle w-32">
    █████        ███                       ███████████
    ░░███        ░░░                       ░█░░░███░░░█
    ░███        ████  █████ █████  ██████ ░   ░███  ░   ██████  ████████  █████████████
    ░███       ░░███ ░░███ ░░███  ███░░███    ░███     ███░░███░░███░░███░░███░░███░░███
    ░███        ░███  ░███  ░███ ░███████     ░███    ░███████  ░███ ░░░  ░███ ░███ ░███
    ░███      █ ░███  ░░███ ███  ░███░░░      ░███    ░███░░░   ░███      ░███ ░███ ░███
    ███████████ █████  ░░█████   ░░██████     █████   ░░██████  █████     █████░███ █████
    ░░░░░░░░░░░ ░░░░░    ░░░░░     ░░░░░░     ░░░░░     ░░░░░░  ░░░░░     ░░░░░ ░░░ ░░░░░
    </pre>
    Type 'help' to see the list of available commands.
    Type 'sumfetch' to display summary.
    Type 'repo' or click <u><a class="text-light-blue dark:text-dark-blue underline" href="{repo}" target="_blank">here</a></u> for the Github repository.
    </pre>
        "#,repo = config.repo).to_owned())
}

pub fn change_theme(_args: Vec<&str>, window: Window) -> Result<String, Error> {
    let document = window.document().expect("window should have a document");
    if document
        .query_selector("#theme")
        .unwrap()
        .unwrap()
        .class_name()
        == "dark"
    {
        document
            .query_selector("#theme")
            .unwrap()
            .unwrap()
            .set_class_name("");
        Ok("Theme changed to light theme".to_owned())
    } else {
        document
            .query_selector("#theme")
            .unwrap()
            .unwrap()
            .set_class_name("dark");
        Ok("Theme changed to dark theme".to_owned())
    }
}

pub async fn execute_command(
    command: String,
    args: Vec<&str>,
    window: Window,
    config: &'static Config<'static>,
    command_list: Vec<&'static str>,
) -> Result<String, Error> {
    match command.as_str() {
        "help" => Ok(help(args, command_list).unwrap()),
        "banner" => Ok(banner(config).unwrap()),
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
        "echo" => Ok(echo(args).unwrap()),
        "sumfetch" => Ok(sumfetch(args, config).unwrap()),
        "theme" => Ok(change_theme(args, window).unwrap()),
        "projects" => Ok(projects(args, config).await.unwrap()),
        "readme" => Ok(read_me(args, config).await.unwrap()),
        "weather" => Ok(weather(args).await.unwrap()),
        "quote" => Ok(quote(args).await.unwrap()),
        &_ => Ok("Unvalid Command...  type 'help' to get started".to_owned()),
    }
}
