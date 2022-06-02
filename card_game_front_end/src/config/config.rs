pub mod config {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub readme_url: String,
        pub title: String,
        pub name: String,
        pub social: Social,
        pub email: String,
        pub ps1_hostname: String,
        pub ps1_username: String,
        pub colors: ThemeColors,
        pub ascii: String,
        pub repo: String,
        pub resume_url: String,
        pub donate_urls: DonateURLs,
    }

    #[derive(Deserialize, Debug)]
    pub struct DonateURLs {
        pub paypal: String,
        pub patreon: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Social {
        pub github: String,
        pub linkedin: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct ThemeColors {
        pub light: Colors,
        pub dark: Colors,
    }

    #[derive(Deserialize, Debug)]
    pub struct Colors {
        pub background: String,
        pub foreground: String,
        pub yellow: String,
        pub green: String,
        pub gray: String,
        pub blue: String,
        pub red: String,
    }
}
