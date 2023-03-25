use lazy_static::lazy_static;
use regex::Regex;

pub fn some_helper_function(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<[\s\S]*>").unwrap();
    }
    RE.is_match(text)
}
