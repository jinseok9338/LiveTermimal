use instant::Instant;

use crate::utils::commands::commands_string::add_string_stream;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    StreamString(String),
    AddLoading,
    StreamTags(Vec<String>),
    CustomCommand(String), // Javascript as String
}

// make

#[derive(Debug, PartialEq, Clone)]
pub struct History {
    pub id: Box<usize>,
    pub date: Box<Instant>,
    pub command: String,
    pub output: String,
    pub operation: Option<Operation>, //Operation,
}

pub fn handle_operation(op: Operation) {
    match op {
        Operation::StreamString(s) => {
            // Handle the StreamString operation
            add_string_stream(&s);
        }
        Operation::CustomCommand(c) => {
            // Handle the CustomCommand operation
        }
        Operation::StreamTags(t) => {}
        Operation::AddLoading => {}
    }
}
