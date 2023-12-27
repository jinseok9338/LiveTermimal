use std::io::Error;
use web_sys::Window;

use crate::config::command_config::config::Config;
use lazy_static::lazy_static;

use regex::Regex;

use super::{
    api_commands::{projects, quote, read_me, weather},
    programs::{
        help::HelpProps, legacy::LegacyProps, open_repo::OpenRepoProps, programs::OutputComponent,
        welcome::WelcomeProps,
    },
    sumfetch::sumfetch,
};

pub type ShellCommandReturnType = Result<Box<OutputComponent>, Error>;

pub fn help(_args: Vec<&str>) -> ShellCommandReturnType {
    let args: Vec<&str> = _args;
    // convert Vec<&str> to Vec<String>
    let args: Vec<String> = args.iter().map(|&s| s.to_string()).collect();
    let output_component = Box::new(OutputComponent::Help(HelpProps { args }));

    Ok(output_component)
}

//Redirection to repo
pub fn repo(_args: Vec<&str>, config: &'static Config<'static>) -> ShellCommandReturnType {
    let output_component = Box::new(OutputComponent::OpenRepo(OpenRepoProps { config }));
    Ok(output_component)
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

    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));

    Ok(output_component)
}

pub fn resume(
    _args: Vec<&str>,
    window: Window,
    config: &'static Config<'static>,
) -> ShellCommandReturnType {
    window.open_with_url(config.resume_url.as_ref()).unwrap();

    let result_string = "Opening resume".to_owned();

    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));
    Ok(output_component)
}

pub fn google(args: Vec<&str>, window: Window) -> ShellCommandReturnType {
    let query = args[1..].join(" ");
    return match query.eq("") {
        true => Ok(Box::new(OutputComponent::Legacy(LegacyProps {
            legacy_output: r#"
                You should provide query
                like this: google facetime 
                "#
            .to_owned(),
        }))),
        _ => {
            window
                .open_with_url(
                    format!("https://google.com/search?q={query}", query = query).as_ref(),
                )
                .unwrap();
            let result_string =
                format!("Searching google for {query}...", query = query).to_owned();

            let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: result_string,
            }));
            Ok(output_component)
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
        true => Ok(Box::new(OutputComponent::Legacy(LegacyProps {
            legacy_output: "You cheeky bastard... You are not allowed to type that".to_string(),
        }))),
        false => Ok(Box::new(OutputComponent::Legacy(LegacyProps {
            legacy_output: query,
        }))),
    }
}

pub fn whoami(_args: Vec<&str>, config: &'static Config<'static>) -> ShellCommandReturnType {
    let result_string = config.ps1_username.to_owned();
    let output_component = Box::new(OutputComponent::Legacy(LegacyProps {
        legacy_output: result_string,
    }));
    Ok(output_component)
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

    Ok(output_component)
}

pub fn welcome_command(_config: &'static Config<'static>) -> ShellCommandReturnType {
    let output_component: Box<OutputComponent> =
        Box::new(OutputComponent::Welcome(WelcomeProps {}));

    Ok(output_component)
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
        let output_component: Box<OutputComponent> =
            Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: result_string,
            }));
        Ok(output_component)
    } else {
        document
            .query_selector("#theme")
            .unwrap()
            .unwrap()
            .set_class_name("dark");
        let result_string = "Theme changed to dark theme".to_owned();
        let output_component: Box<OutputComponent> =
            Box::new(OutputComponent::Legacy(LegacyProps {
                legacy_output: result_string,
            }));
        Ok(output_component)
    }
}

pub async fn execute_command(
    command: String,
    args: Vec<&str>,
    window: Window,
    config: &'static Config<'static>,
) -> Result<Box<OutputComponent>, Error> {
    match command.as_str() {
        "help" => Ok(help(args).unwrap()),
        "banner" => Ok(banner(config).unwrap()),
        "about" => Ok(about(args, config).unwrap()),
        "repo" => Ok(repo(args, config).unwrap()),
        "resume" => Ok(resume(args, window, config).unwrap()),
        "google" => Ok(google(args, window).unwrap()),
        "whoami" => Ok(whoami(args, config).unwrap()),
        "echo" => Ok(echo(args).unwrap()),
        "sumfetch" => Ok(sumfetch(args, config).unwrap()),
        "theme" => Ok(change_theme(args, window).unwrap()),
        "projects" => Ok(projects(args, config).await.unwrap()),
        "readme" => Ok(read_me(args, config).await.unwrap()),
        "weather" => Ok(weather(args).await.unwrap()),
        "quote" => Ok(quote(args).await.unwrap()),
        &_ => Ok(Box::new(OutputComponent::Legacy(LegacyProps {
            legacy_output: "Unvalid Command...  type 'help' to get started".to_owned(),
        }))),
    }
}
