pub mod config {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Config<'a> {
        pub readme_url: &'a str,
        pub title: &'a str,
        pub name: &'a str,
        pub social: Social<'a>,
        pub email: &'a str,
        pub ps1_hostname: &'a str,
        pub ps1_username: &'a str,
        pub colors: ThemeColors<'a>,
        pub ascii: &'a str,
        pub repo: &'a str,
        pub resume_url: &'a str,
        pub donate_urls: DonateURLs<'a>,
        pub group: &'a str,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct DonateURLs<'a> {
        pub paypal: &'a str,
        pub patreon: &'a str,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Social<'a> {
        pub github: &'a str,
        pub linkedin: &'a str,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct ThemeColors<'a> {
        #[serde(borrow)]
        pub light: Colors<'a>,
        #[serde(borrow)]
        pub dark: Colors<'a>,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Colors<'a> {
        pub background: &'a str,
        pub foreground: &'a str,
        pub yellow: &'a str,
        pub green: &'a str,
        pub gray: &'a str,
        pub blue: &'a str,
        pub red: &'a str,
    }
}
