pub mod config {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone, PartialEq)]
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

    impl Config {
        pub fn new(self: &Self) -> Self {
            Self {
                ascii: self.ascii.to_owned(),
                colors: self.colors.to_owned(),
                donate_urls: self.donate_urls.to_owned(),
                email: self.email.to_owned(),
                name: self.name.to_owned(),
                ps1_hostname: self.ps1_hostname.to_owned(),
                ps1_username: self.ps1_username.to_owned(),
                readme_url: self.readme_url.to_owned(),
                repo: self.repo.to_owned(),
                resume_url: self.resume_url.to_owned(),
                social: self.social.to_owned(),
                title: self.title.to_owned(),
            }
        }
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct DonateURLs {
        pub paypal: String,
        pub patreon: String,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Social {
        pub github: String,
        pub linkedin: String,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct ThemeColors {
        pub light: Colors,
        pub dark: Colors,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
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

// readme_url: "https://raw.githubusercontent.com/cveinnt/cveinnt/master/README.md"
// .to_owned(),
// title: "LiveTerm".to_owned(),
// name: "John Doe".to_owned(),
// ascii: "liveterm".to_owned(),
// social: Social {
// github: "github".to_owned(),
// linkedin: "linkedin".to_owned(),
// },
// email: "contact@email.com".to_owned(),
// ps1_hostname: "liveterm".to_owned(),
// ps1_username: "visitor".to_owned(),
// repo: "https://github.com/Cveinnt/LiveTerm".to_owned(),
// resume_url: "https://upload.wikimedia.org/wikipedia/commons/c/cc/Resume.pdf"
// .to_owned(),
// donate_urls: DonateURLs {
// paypal: "https://paypal.me/cveinnt".to_owned(),
// patreon: "https://patreon.com/cveinnt".to_owned(),
// },
// colors: ThemeColors {
// light: Colors {
//     background: "#FBF1C9".to_owned(),
//     foreground: "#3C3836".to_owned(),
//     yellow: "#D79921".to_owned(),
//     green: "#98971A".to_owned(),
//     gray: "#7C6F64".to_owned(),
//     blue: "#458588".to_owned(),
//     red: "#CA2124".to_owned(),
// },
// dark: Colors {
//     background: "#2E3440".to_owned(),
//     foreground: "#E5E9F0".to_owned(),
//     yellow: "#5E81AC".to_owned(),
//     green: "#A3BE8C".to_owned(),
//     gray: "#88C0D0".to_owned(),
//     blue: "#EBCB8B".to_owned(),
//     red: "#BF616A".to_owned(),
// },
// },
