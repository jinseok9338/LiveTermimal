use crate::utils::commands::programs::programs::OutputComponent;
use instant::Instant;
#[derive(PartialEq, Clone)]
pub struct History {
    pub id: Box<usize>,
    pub date: Box<Instant>,
    pub command: String,
    pub output: Box<OutputComponent>,
}
