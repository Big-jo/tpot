impl TaskPopUp {
    pub fn new() -> Self {
        TaskPopUp {
            active: false,
            input: String::new(),
            input_mode: InputMode::Normal,
            command: Command::None,
            selected_task: None,
            list_state: ListState::default(),
        }
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        if self.active {
            self.input_mode = InputMode::Normal;
            self.input.clear();
        }
    }

    pub fn select_next(&mut self, task_count: usize) {
        if task_count == 0 {
            ()
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= task_count - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.list_state.select(Some(i));
    }

    pub fn select_previous(&mut self, task_count: usize) {
        if task_count == 0 {
            ()
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    task_count - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.list_state.select(Some(i));
        self.selected_task = Some(i);
    }

    pub fn start_editing(&mut self) {
        // self.input_mode = InputMode::Editing;
    }

    pub fn stop_editing(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => {
                match key_event.code {
                    KeyCode::Char('e') => {
                        if let Some(idx) = self.selected_task {
                            // self.input_mode = InputMode::Editing;
                            self.command = Command::Update(idx);
                            self.input.clear();
                        }
                    }
                    KeyCode::Char('c') => {
                        // self.input_mode = InputMode::Editing;
                        self.command = Command::Create;
                        self.input.clear();
                    }
                    // KeyCode::Esc => {
                    //     return Some(Command::None); // Signal to close the popup
                    // }
                    _ => {}
                }
            }
            // InputMode::Editing => match key_event.code {
            //     KeyCode::Enter => match &self.command {
            //         Command::Create => {
            //             if !self.input.is_empty() {
            //                 self.input_mode = InputMode::Normal;
            //             }
            //         }
            //         Command::Update(_) => {
            //             if !self.input.is_empty() {
            //                 self.input_mode = InputMode::Normal;
            //             }
            //         }
            //         _ => {}
            //     },
            //     KeyCode::Esc => {
            //         self.input_mode = InputMode::Normal;
            //         self.input.clear();
            //     }
            //     KeyCode::Char(c) => {
            //         self.input.push(c);
            //     }
            //     KeyCode::Backspace => {
            //         self.input.pop();
            //     }
            //     _ => {}
            // },
        }
    }
}


