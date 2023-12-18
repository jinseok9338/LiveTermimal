use yew::UseStateHandle;

use super::interface::History;

pub fn clear_history(history_handler: UseStateHandle<Vec<History>>) {
    let empty_vector = Vec::new();
    history_handler.set(empty_vector)
}

pub fn set_history(
    history_handler: UseStateHandle<Vec<History>>,
    command_handler: UseStateHandle<String>,
    value: String,
) {
    let command = &*command_handler;
    let new_history = History {
        command: command.to_owned(),
        id: Box::new((*command_handler).len()),
        output: value,
        date: Box::new(instant::Instant::now()),
    };

    let mut old_history = (*history_handler).clone();
    old_history.push(new_history);
    history_handler.set(old_history)
}
