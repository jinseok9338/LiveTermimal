use std::io::Error;
use web_sys::Window;

use crate::{components::history::interface::Operation, config::command_config::config::Config};
use lazy_static::lazy_static;

use regex::Regex;

use super::{
    api_commands::{projects, quote, read_me, weather},
    programs::{help::HelpProps, legacy::LegacyProps, programs::OutputComponent},
    sumfetch::sumfetch,
};

pub type ShellCommandReturnType = Result<(Box<OutputComponent>, Option<Operation>), Error>;

pub fn help(_args: Vec<&str>) -> ShellCommandReturnType {
    let args: Vec<&str> = _args;
    // convert Vec<&str> to Vec<String>
    let args: Vec<String> = args.iter().map(|&s| s.to_string()).collect();
    let output_component = Box::new(OutputComponent::Help(HelpProps { args }));

    Ok((output_component, None))
}

//Redirection to repo
pub fn repo(
    _args: Vec<&str>,
    window: Window,
    config: &'static Config<'static>,
) -> ShellCommandReturnType {
    window.open_with_url(config.repo.as_ref()).unwrap();
    let result_string = "Opening Github repository...".to_owned();
    let operation = None;
    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));
    Ok((output_component, operation))
}

//About
pub fn about(_args: Vec<&str>, config: &'static Config<'static>) -> ShellCommandReturnType {
    let result_string = format!(
        r#"Hi, I am {name}.
        Welcome to my website!
        More about me:
        'sumfetch' - short summary.
        'resume' - my latest resume.
        'readme' - my github readme.`
    "#,
        name = config.name
    )
    .to_owned();
    let operation = None;

    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));

    Ok((output_component, operation))
}

pub fn resume(
    _args: Vec<&str>,
    window: Window,
    config: &'static Config<'static>,
) -> ShellCommandReturnType {
    window.open_with_url(config.resume_url.as_ref()).unwrap();

    let result_string = "Opening resume".to_owned();
    let operation = None;
    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));
    Ok((output_component, operation))
}

pub fn donate(_args: Vec<&str>) -> ShellCommandReturnType {
    let result_string =r#"
        thank you for your interest.
        here are the ways you can support my work:
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.paypal}" target="_blank">paypal</a></u>
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.patreon}" target="_blank">patreon</a></u>
        "#.to_owned();

    let operation = None;
    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));
    Ok((output_component, operation))
}

pub fn google(args: Vec<&str>, window: Window) -> ShellCommandReturnType {
    let query = args[1..].join(" ");
    return match query.eq("") {
        true => Ok((
            Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: r#"
                You should provide query
                like this: google facetime 
                "#
                .to_owned(),
            })),
            None,
        )),
        _ => {
            window
                .open_with_url(
                    format!("https://google.com/search?q={query}", query = query).as_ref(),
                )
                .unwrap();
            let result_string =
                format!("Searching google for {query}...", query = query).to_owned();
            let operation = None;
            let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: result_string,
            }));
            Ok((output_component, operation))
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
pub fn echo(args: Vec<&str>) -> ShellCommandReturnType {
    let query = args[1..].join(" ");
    match some_helper_function(&query) {
        true => Ok((
            Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: "You cheeky bastard... You are not allowed to type that".to_string(),
            })),
            None,
        )),
        false => Ok((
            Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: query,
            })),
            None,
        )),
    }
}

pub fn whoami(_args: Vec<&str>, config: &'static Config<'static>) -> ShellCommandReturnType {
    let result_string = config.ps1_username.to_owned();
    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));
    Ok((output_component, None))
}

pub fn banner(config: &'static Config<'static>) -> ShellCommandReturnType {
    let result_string = format!(r#"
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
        "#,repo = config.repo).to_owned();
    let output_component: Box<OutputComponent> = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));
    let operation = None;
    Ok((output_component, operation))
}

pub fn welcome_string(_config: &'static Config<'static>) -> ShellCommandReturnType {
    let result_string = format!(
        r#"
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
    </pre>"#
    )
    .to_owned();

    let operation_string = r"
    Hello, fellow users! Welcome to my portfolio website. This was made with Rust and WebAssembly.
    I hope you enjoy the experience. There are plenty of commands you can try such as sumfetch, repo, and more. 
    If you don't know where to begin, type help to see the list of commands. Or type 'sumfetch' to display summary.
    Any feedback is appreciated.
    "
        .to_owned();

    let operation = Some(Operation::StreamString(operation_string));
    let output_component: Box<OutputComponent> = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));
    Ok((output_component, operation))
}

pub fn change_theme(_args: Vec<&str>, window: Window) -> ShellCommandReturnType {
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
        let result_string = "Theme changed to light theme".to_owned();
        let operation = None;
        let output_component: Box<OutputComponent> =
            Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: result_string,
            }));
        Ok((output_component, operation))
    } else {
        document
            .query_selector("#theme")
            .unwrap()
            .unwrap()
            .set_class_name("dark");

        let result_string = "Theme changed to dark theme".to_owned();
        let operation = None;
        let output_component: Box<OutputComponent> =
            Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: result_string,
            }));
        Ok((output_component, operation))
    }
}

pub async fn execute_command(
    command: String,
    args: Vec<&str>,
    window: Window,
    config: &'static Config<'static>,
) -> Result<(Box<OutputComponent>, Option<Operation>), Error> {
    match command.as_str() {
        "help" => Ok(help(args).unwrap()),
        "banner" => Ok(banner(config).unwrap()),
        "about" => Ok(about(args, config).unwrap()),
        "repo" => Ok(repo(args, window, config).unwrap()),
        "resume" => Ok(resume(args, window, config).unwrap()),
        "donate" => Ok(donate(args).unwrap()),
        "google" => Ok(google(args, window).unwrap()),
        "whoami" => Ok(whoami(args, config).unwrap()),
        "echo" => Ok(echo(args).unwrap()),
        "sumfetch" => Ok(sumfetch(args, config).unwrap()),
        "theme" => Ok(change_theme(args, window).unwrap()),
        "projects" => Ok(projects(args, config).await.unwrap()),
        "readme" => Ok(read_me(args, config).await.unwrap()),
        "weather" => Ok(weather(args).await.unwrap()),
        "quote" => Ok(quote(args).await.unwrap()),
        &_ => Ok((
            Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: "Unvalid Command...  type 'help' to get started".to_owned(),
            })),
            None,
        )),
    }
}
