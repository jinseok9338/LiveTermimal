use std::fmt::Error;

use crate::config::config::config::Config;

pub fn sumfetch(_args: Vec<&str>, config: Config) -> Result<String, Error> {
    if config.ascii == "jason".to_owned() {
        Ok(format!(r#"

/*///////****////***///****///****////////           sumfetch: summary display
/****/////***********/***********/////**//          -----------
////**************████*█████████***//****/           ABOUT
/////********████████           ██*****//            {name}
*********████████                ████*//*/          ﰩ {ps1_hostname}
/***██████                          ████**           <u><a href="{resume_url}" target="_blank">resume</a></u>
/***██           ██  ██               █░░░            <u><a href="{repo}" target="_blank">Github repo</a></u>
***██                            ██████░░@           -----------
/**██      ████                █ ████░░/!@           CONTACT
****██                        █████░░//%%!           <u><a href="mailto:{email}" target="_blank">{email}</a></u>
*****████                   █ ████░░***@@#           <u><a href="https://github.com/{github}" target="_blank">github.com/{github}</a></u>
///***████████#███        ███##██░░**/////           <u><a href="https://linkedin.com/in/{linkedin}" target="_blank">linkedin.com/in/{linkedin}</a></u>
**********███##██#       ██##███░░░******?          
**///****///**████        ████░░░**//////?          
/////********█████        █████░░░*****//!          
///****////****██       #  ██░░░**////****          
"#,
 name = config.name,
 ps1_hostname= config.ps1_hostname,
 resume_url = config.resume_url,
  repo = config.repo,
  email = config.email,
  github = config.social.github,
  linkedin = config.social.linkedin,
  ).to_owned())
    } else {
        Ok(format!(r#"

                  ▄██████████████▄                  sumfetch
               ▄██▀ ▄██▀███▀██▄ ▀▀██▄              -----------
             ██▀  ▄█▀   ▐██  ▀██    ██▄             ABOUT
           ▄████████████████████████████            {name}
          ██     ███    ▐██    ▐██     ██           <u><a href="{resume_url}" target="_blank">resume</a></u>
       ▐█▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█       爵 <u><a href="{repo}" target="_blank">Github repo</a></u>
       ▐█                                 ▐█       -----------
       ▐█        > L I V E T E R M        ▐█        CONTACT
       ▐█                                 ▐█        <u><a href="mailto:{email}" target="_blank">{email}</a></u>
       ▐█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█        <u><a href="https://github.com/{github}" target="_blank">github.com/{github}</a></u>
          ██      ▐██    ██    ▐██     ██           <u><a href="https://linkedin.com/in/{linkedin}" target="_blank">linkedin.com/in/{linkedin}</a></u>
           █████████████████████████████           -----------
             ███   ▐██   ██   ███   ██▀             DONATE
               ▀██▄▄ ▀██▄██▄███▄▄██▀                <u><a href="{paypal}" target="_blank">{paypal}</a></u>
                   ▀███████████▀▀                   <u><a href="{patreon}" target="_blank">{patreon}</a></u>
"#,
name = config.name,
resume_url = config.resume_url,
 repo = config.repo,
 email = config.email,
 github = config.social.github,
 linkedin = config.social.linkedin,
 paypal = config.donate_urls.paypal,
 patreon = config.donate_urls.patreon,).to_owned())
    }
}
