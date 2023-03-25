use std::io::Error;

use web_sys::Window;

use crate::config::config::config::Config;




use super::{
    api_commands::{projects, quote, read_me, weather},
    sumfetch::sumfetch, helper::some_helper_function,
};

pub fn help(_args: &[&str], command_list: Vec<&'static str>) -> String {
    let mut result_string = String::new();
    for (i, command) in command_list.into_iter().enumerate() {
        if i % 7 == 0 {
            result_string.push_str(&format!("{command}\n"));
        } else {
            result_string.push_str(&format!("{command} ", ));
        }
    }

    format!(
        "Welcome! Here are all the available commands:
        \n{result_string}\n
        [tab]: trigger completion.
        [ctrl+l]/clear: clear terminal.\n
        Type 'sumfetch' to display summary."
        
    )
}

//Redirection to repo
pub fn repo(_args: &[&str], window: &Window, config: &Config) -> String {
    window.open_with_url(config.repo.as_ref()).unwrap();

    "Opening Github repository...".to_owned()
}

//About
pub fn about(_args: &[&str], config: &Config) -> String {
   format!(
        r#"Hi, I am {name}.
        Welcome to my website!
        More about me:
        'sumfetch' - short summary.
        'resume' - my latest resume.
        'readme' - my github readme.`
    "#,
        name = config.name
    )
 
}

pub fn resume(_args: &[&str], window: &Window, config: &Config) -> String {
    window.open_with_url(config.resume_url.as_ref()).unwrap();

   "Opening resume".to_owned()
}

pub fn donate(_args: &[&str]) -> String {
    r#"
        thank you for your interest.
        here are the ways you can support my work:
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.paypal}" target="_blank">paypal</a></u>
        - <u><a class="text-light-blue dark:text-dark-blue underline" href="${config.donate_urls.patreon}" target="_blank">patreon</a></u>
        "#.to_owned()
}

pub fn google(args: &[&str], window: &Window) -> String {
    let query = args[1..].join(" ");
    return if query.eq("") {
        r#"
        You should provide query
        like this: google facetime 
        "#
        .to_owned()
    } else {
        window
            .open_with_url(format!("https://google.com/search?q={query}").as_ref())
            .unwrap();
        format!("Searching google for {query}...")
    }
}

pub fn duckduckgo(args: &[&str], window: &Window) -> String {
    let query = args[1..].join(" ");
    return match query.eq("") {
        true => r#"
        You should provide query
        like this: duckduckgo facetime 
        "#
        .to_owned(),
        _ => {
            window
                .open_with_url(format!("https://duckduckgo.com/?q={query}" ).as_ref())
                .unwrap();
            format!("Searching duckduckgo for {query}...")
        }
    };
}

pub fn bing(args: &[&str], window: &Window) -> String {
    let query = args[1..].join(" ");
    return if query.eq("") {
        r#"
        You should provide query
        like this: bing facetime 
        "#
        .to_owned()
    } else {
        window
            .open_with_url(format!("https://bing.com/search?q={query}").as_ref())
            .unwrap();
        format!("Searching bing for {query}...")
    }
}

pub fn reddit(args: &[&str], window: &Window) -> String {
    let query = args[1..].join(" ");
    return if query.eq("") {
        r#"
        You should provide query
        like this: reddit facetime 
        "#
        .to_owned()
    } else {
        window
            .open_with_url(format!("https://reddit.com/search/?q={query}").as_ref())
            .unwrap();
        format!("Searching reddit for {query}...")
    }
}



//Typical linux Commands
pub fn echo(args: Vec<&str>) -> String {
    let query = args[1..].join(" ");
    if some_helper_function(&query) {
        "You cheeky bastard... You are not allowed to type that".to_string()
    } else {
        query
    }
}

pub fn whoami(_args: Vec<&str>, config: &Config) -> String {
    config.ps1_username.clone()
}

// This will be done in the future
pub fn ls(_args: &[&str]) -> String {
    r#"
    I
    don't 
    know 
    how 
    to add file system in 
    webAssemly"#
        .to_owned()
}

// This will be done in the future
pub fn cd(_args: &[&str]) ->String {
    r#"
    I
    don't 
    know 
    how 
    to add file system in 
    webAssemly"#
        .to_owned()
}

pub fn banner(config: &Config) -> String {
    format!(r#"
    <pre>
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
    Type 'repo' or click <u><a class="text-light-blue dark:text-dark-blue underline" href="{0}" target="_blank">here</a></u> for the Github repository.
    </pre>
        "#, config.repo)
}

pub fn change_theme(_args: Vec<&str>, window: Window) -> String {
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
        "Theme changed to light theme".to_owned()
    } else {
        document
            .query_selector("#theme")
            .unwrap()
            .unwrap()
            .set_class_name("dark");
        "Theme changed to dark theme".to_owned()
    }
}

pub async fn execute_command(
    command: String,
    args: Vec<&str>,
    window: Window,
    config: Config,
    command_list: Vec<&'static str>,
) -> Result<String, Error> {
    match command.as_str() {
        "help" => Ok(help(&args, command_list)),
        "banner" => Ok(banner(&config)),
        "about" => Ok(about(&args, &config)),
        "bing" => Ok(bing(&args, &window)),
        "repo" => Ok(repo(&args, &window, &config)),
        "resume" => Ok(resume(&args, &window, &config)),
        "donate" => Ok(donate(&args)),
        "google" => Ok(google(&args, &window)),
        "duckduckgo" => Ok(duckduckgo(&args, &window)),
        "reddit" => Ok(reddit(&args, &window)),
        "whoami" => Ok(whoami(args, &config)),
        "ls" => Ok(ls(&args)),
        "cd" => Ok(cd(&args)),
        "echo" => Ok(echo(args)),
        "sumfetch" => Ok(sumfetch(&args, config)),
        "theme" => Ok(change_theme(args, window)),
        "projects" => Ok(projects(args, config).await.unwrap()),
        "readme" => Ok(read_me(args, config).await.unwrap()),
        "weather" => Ok(weather(args).await.unwrap()),
        "quote" => Ok(quote(args).await.unwrap()),
        &_ => Ok("Unvalid Command...  type 'help' to get started".to_owned()),
    }
}
