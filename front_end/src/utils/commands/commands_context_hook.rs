use crate::config::command_config::config::Colors;
use crate::config::command_config::config::Config;
use crate::config::command_config::config::DonateURLs;
use crate::config::command_config::config::Social;
use crate::config::command_config::config::ThemeColors;

pub const COMMAND_LIST: [&str; 21] = [
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
    "sumfetch",
    "theme",
    "clear",
    "projects",
    "readme",
    "weather",
    "quote",
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
