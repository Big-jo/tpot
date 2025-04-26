use ratatui::widgets::ListState;

pub enum InputMode {
    Normal,
    Editing,
}
pub enum Command {
    Create,
    Update(usize),
    Delete(usize),
    None,
}

pub struct TaskPopUp {
    active: bool,
    input: String,
    input_mode: InputMode,
    command: Command,
    selected_task: Option<usize>,
    list_state: ListState,
}
