use yew::UseStateHandle;

use crate::utils::commands::programs::programs::OutputComponent;

use super::interface::History;

pub fn clear_history(history_handler: UseStateHandle<Vec<History>>) {
    let empty_vector = Vec::new();
    history_handler.set(empty_vector)
}

pub fn set_history(
    history_handler: UseStateHandle<Vec<History>>,
    command: String,
    value: Box<OutputComponent>,
    operation: Option<super::interface::Operation>,
) {
    let new_history = History {
        command: command.to_owned(),
        id: Box::new(command.len()),
        output: value,
        date: Box::new(instant::Instant::now()),
        operation,
    };

    let mut old_history = (*history_handler).clone();
    old_history.push(new_history);
    history_handler.set(old_history)
}
