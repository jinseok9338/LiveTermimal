use instant::Instant;

use crate::utils::commands::programs::programs::OutputComponent;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    StreamString(String),
}

// make

#[derive(PartialEq, Clone)]
pub struct History {
    pub id: Box<usize>,
    pub date: Box<Instant>,
    pub command: String,
    pub output: Box<OutputComponent>,
    pub operation: Option<Operation>, //Operation,
}
