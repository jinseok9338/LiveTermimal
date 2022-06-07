use instant::Instant;
use web_sys::DateTimeValue;
#[derive(Debug, PartialEq, Clone)]
pub struct History {
    pub id: usize,
    pub date: Instant,
    pub command: String,
    pub output: String,
}
