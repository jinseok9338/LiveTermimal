use crate::config::command_config::config::Colors;
use crate::config::command_config::config::Config;
use crate::config::command_config::config::DonateURLs;
use crate::config::command_config::config::Social;
use crate::config::command_config::config::ThemeColors;

pub const COMMAND_LIST_DESCRIPTION: [&str; 13] = [
    "<div class='flex'><p class='text-description font-bold'>banner</p> <p class='description'> - This shows the banner</p></div>",
    "<div class='flex'><p class='text-description font-bold'>help</p> <p class='description'> - This will show help </p></div>",
    "<div class='flex'><p class='text-description font-bold'>repo</p> <p class='description'> - This will show repo </p></div>",
    "<div class='flex'><p class='text-description font-bold'>resume</p> <p class='description'> - This will show resume </p></div>",
    "<div class='flex'><p class='text-description font-bold'>google</p> <p class='description'> - This will show search result in google </p></div>",
    "<div class='flex'><p class='text-description font-bold'>echo</p> <p class='description'> - This will repeat the word you type </p></div>",
    "<div class='flex'><p class='text-description font-bold'>whoami</p> <p class='description'> - This will show whoami </p></div>",
    "<div class='flex'><p class='text-description font-bold'>sumfetch</p> <p class='description'> - This will show summary </p></div>",
    "<div class='flex'><p class='text-description font-bold'>theme</p> <p class='description'> - This will change theme </p></div>",
    "<div class='flex'><p class='text-description font-bold'>clear</p> <p class='description'> - This will clear commands and history </p></div>",
    "<div class='flex'><p class='text-description font-bold'>projects</p> <p class='description'> - This will show projects of Jinseok's repo </p></div>",
    "<div class='flex'><p class='text-description font-bold'>weather</p> <p class='description'> - This will show the weather of the city or country </p></div>",
    "<div class='flex'><p class='text-description font-bold'>quote</p> <p class='description'> - This will show random quote </p></div>",
];
pub const COMMAND_LIST_VEC: [&str; 13] = [
    "banner", "help", "repo", "resume", "google", "echo", "whoami", "sumfetch", "theme", "clear",
    "projects", "weather", "quote",
];

pub const CONFIG: Config = Config {
    readme_url: "https://raw.githubusercontent.com/jinseok9338/CardGame/master/README.md",

    title: "JTerm",
    name: "Jason Jin (Jinseok)",
    ascii: "jason",
    social: Social {
        github: "jinseok9338",
        linkedin: "linkedin",
    },
    email: "jinseok9338@gmail.com",
    ps1_hostname: "hostName",
    ps1_username: "Jason Jin",
    repo: "https://github.com/jinseok9338/CardGame",
    resume_url: "https://jinseokseo.tiiny.site/",
    donate_urls: DonateURLs {
        paypal: "https://paypal.me",
        patreon: "https://patreon.com",
    },
    colors: ThemeColors {
        light: Colors {
            background: "#FBF1C9",
            foreground: "#3C3836",
            yellow: "#D79921",
            green: "#98971A",
            gray: "#7C6F64",
            blue: "#458588",
            red: "#CA2124",
        },
        dark: Colors {
            background: "#2E3440",
            foreground: "#E5E9F0",
            yellow: "#5E81AC",
            green: "#A3BE8C",
            gray: "#88C0D0",
            blue: "#EBCB8B",
            red: "#BF616A",
        },
    },
};
