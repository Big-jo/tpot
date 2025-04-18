use crossterm::event::KeyEvent;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    prelude::*,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Gauge, ListState, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};
use std::{io, sync::mpsc, thread, time::Duration};

// ====== EVENT HANDLING ======

enum Event {
    Input(crossterm::event::KeyEvent),
    Progress(f64),
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

fn run_background_thread(tx: mpsc::Sender<Event>) {
    let mut progress = 0_f64;
    let increment = 0.01_f64;
    loop {
        std::thread::sleep(Duration::from_millis(100));
        progress += increment;
        progress = progress.min(1_f64);
        tx.send(Event::Progress(progress)).unwrap();
    }
}

// ====== DATA STRUCTURES ======

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityData {
    friends: Vec<UserData>,
    profile: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    user_name: String,
    tasks: Vec<TaskData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskData {
    task_name: String,
    progress: f64,
    timer: f64,
    id: i32,
}

pub struct ActivityProgressBar {
    progress: f64,
    timer: f64,
    color: Color,
    user_name: String,
    task_name: String,
}

pub struct App {
    exit: bool,
    progress_bar_color: Color,
    background_progress: f64,
    data: ActivityData,
    active_tab: usize,
    popup: TaskPopUp,
}

// ====== WIDGET IMPLEMENTATIONS ======

impl Widget for ActivityProgressBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let activity_block = Layout::vertical([
            Constraint::Length(1), // Header
            Constraint::Min(3),    // Progress bar
        ]);

        let [activity_text_area, activity_progress_area] = activity_block.areas(area);

        // Header text with improved styling
        Paragraph::new(Line::from(vec!["Activity".bold().yellow()]))
            .render(activity_text_area, buf);

        let horizontal_progress_layout =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]);

        let [progress_area, time_area] = horizontal_progress_layout.areas(activity_progress_area);

        let title_text = if !self.user_name.is_empty() {
            format!(" @{} 🔔 {}", self.user_name, self.task_name)
        } else {
            "Current Task".to_string()
        };

        // Enhance the progress bar block
        let progress_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(title_text)
            .title_style(Style::default().fg(Color::White).bold());

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.color))
            .block(progress_block)
            .label(format!("{:.0}%", self.progress * 100_f64))
            .ratio(self.progress);

        progress_bar.render(progress_area, buf);

        // Enhance the time block
        let time_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Time ")
            .title_style(Style::default().fg(Color::White).bold());

        let time_text = format!("{:.0}s left", self.timer * 100_f64);

        Paragraph::new(Line::from(vec![time_text.fg(Color::LightGreen)]))
            .block(time_block)
            .centered()
            .render(time_area, buf);
    }
}

impl Widget for TaskData {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let task_layout =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]);

        let [task_area, time_area] = task_layout.areas(area);

        // Task name with improved style
        Paragraph::new(Line::from(vec![
            "• ".fg(Color::Yellow),
            self.task_name.fg(Color::White),
        ]))
        .render(task_area, buf);

        // Time remaining with improved style
        let time = format!("{:.0}s left", self.timer);
        Paragraph::new(Line::from(time).fg(Color::LightGreen))
            .right_aligned()
            .render(time_area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Main app frame with improved title
        let app_title = format!(
            " TPot - Pomodoro Tracker ({}) ",
            match self.active_tab {
                0 => "Dashboard",
                1 => "Settings",
                _ => "Dashboard",
            }
        );

        let outer_layout = Block::default()
            .title(app_title)
            .title_style(Style::default().fg(Color::Green).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));

        let outer_inner_area = outer_layout.inner(area);
        outer_layout.render(area, buf);

        // Main horizontal layout
        let inner_layout =
            Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]);

        let [left_area, right_area] = inner_layout.areas(outer_inner_area);

        // Left side vertical layout
        let profile_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [profile_area, friends_area] = profile_layout.areas(left_area);

        // Profile section with improved styling
        let profile_block = Block::default()
            .title(" My Profile ")
            .title_style(Style::default().fg(Color::LightBlue).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        profile_block.render(profile_area, buf);

        // Friends section with improved styling
        let friends_block = Block::default()
            .title(" Friends' Activity ")
            .title_style(Style::default().fg(Color::LightMagenta).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        friends_block.render(friends_area, buf);

        // Right side sections
        let details_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [upper_details, lower_details] = details_layout.areas(right_area);

        // Upper details section with improved styling
        let upper_details_block = Block::default()
            .title(" Stats & Details ")
            .title_style(Style::default().fg(Color::LightCyan).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        upper_details_block.render(upper_details, buf);

        // Lower details section with improved styling
        let lower_details_block = Block::default()
            .title(" Upcoming Tasks ")
            .title_style(Style::default().fg(Color::LightYellow).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        lower_details_block.render(lower_details, buf);

        // Render profile progress bar
        let profile_progress_bar = ActivityProgressBar {
            progress: self.background_progress,
            timer: 1.0 - self.background_progress,
            color: self.progress_bar_color,
            user_name: "".to_string(),
            task_name: self
                .data
                .profile
                .tasks
                .first()
                .map_or("No task", |t| &t.task_name)
                .to_string(),
        };

        profile_progress_bar.render(
            Rect {
                x: profile_area.x + 2,
                y: profile_area.y + 2,
                width: profile_area.width - 4,
                height: 3,
            },
            buf,
        );

        Block::default()
            .title(" Tasks ")
            .title_style(Style::default().fg(Color::LightCyan).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .render(
                Rect {
                    x: profile_area.x + 2,
                    y: profile_area.y + 5,
                    width: profile_area.width - 4,
                    height: profile_area.height - 7,
                },
                buf,
            );

        // Render profile tasks
        if !self.data.profile.tasks.is_empty() {
            let profile_task_area = profile_area.inner(Margin {
                horizontal: 2,
                vertical: 6,
            });
            let task_count: usize = self.data.profile.tasks.len();

            let profile_task_layout = Layout::vertical(vec![Constraint::Length(2); task_count])
                .split(profile_task_area.inner(Margin {
                    horizontal: 2,
                    vertical: 1,
                }));

            for (task, area) in self
                .data
                .profile
                .tasks
                .iter()
                .zip(profile_task_layout.iter())
            {		
								let mut count: usize = 0;
                TaskData {
                    task_name: task.task_name.clone(),
                    progress: task.progress,
                    timer: task.timer,
                    id: count as i32,
                }
                .render(*area, buf);

							count += 1;
            }
        }

        // Render friends' activities
        if !self.data.friends.is_empty() {
            let row_count = self.data.friends.len();
            let friends_inner_area = friends_area.inner(Margin {
                horizontal: 1,
                vertical: 1,
            });

            let friend_group_progress_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(5); row_count])
                .split(friends_inner_area);

            for (user_data, area) in self
                .data
                .friends
                .iter()
                .zip(friend_group_progress_layout.iter())
            {
                if let Some(task) = user_data.tasks.first() {
                    let progress_bar = ActivityProgressBar {
                        progress: task.progress.clamp(0.1, 1.0),
                        timer: 1.0 - task.timer,
                        color: Color::DarkGray,
                        user_name: user_data.user_name.clone(),
                        task_name: task.task_name.clone(),
                    };
                    progress_bar.render(
                        {
                            Rect {
                                x: area.x,
                                y: area.y,
                                width: area.width,
                                height: 3,
                            }
                        },
                        buf,
                    );
                }
            }
        }

        // Add footer with keyboard shortcuts
        let footer_text = " [Q] Quit | [C] Change Color | [Tab] Switch View ";
        let footer_style = Style::default().fg(Color::DarkGray);

        let footer_area = Rect {
            x: area.x + 2,
            y: area.height - 1,
            width: area.width - 4,
            height: 1,
        };

        Paragraph::new(Line::from(footer_text).style(footer_style)).render(footer_area, buf);
    }
}

// ====== PopUp Implementation ======
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
    selected_task: Option<i32>,
    list_state: ListState,
}

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
        self.selected_task = 
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
        self.input_mode = InputMode::Editing;
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
                            self.input_mode = InputMode::Editing;
                            self.command = Command::Update(idx);
                            self.input.clear();
                            return None;
                        }
                    }
                    KeyCode::Char('c') => {
                        self.input_mode = InputMode::Editing;
                        self.command = Command::Create;
                        self.input.clear();
                        return None;
                    }
                    KeyCode::Char('d') => {
                        if let Some(idx) = self.selected_task {
                            return Some(Command::Delete(idx));
                        }
                    }
                    KeyCode::Esc => {
                        return Some(Command::None); // Signal to close the popup
                    }
                    _ => {}
                }
            }
            InputMode::Editing => match key_event.code {
                KeyCode::Enter => match &self.command {
                    Command::Create => {
                        if !self.input.is_empty() {
                            let result = self.command.clone();
                            self.input_mode = InputMode::Normal;
                        }
                    }
                    Command::Update(_) => {
                        if !self.input.is_empty() {
                            let result = self.command.clone();
                            self.input_mode = InputMode::Normal;
                        }
                    }
                    _ => {}
                },
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input.clear();
                }
                KeyCode::Char(c) => {
                    self.input.push(c);
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                _ => {}
            },
        }
        None
    }
}

// ====== APP IMPLEMENTATION ======

impl App {
    fn new(data: ActivityData) -> Self {
        App {
            exit: false,
            progress_bar_color: Color::Green,
            background_progress: 0_f64,
            data,
            active_tab: 0,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            match rx.recv().unwrap() {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Progress(progress) => self.background_progress = progress,
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                crossterm::event::KeyCode::Char('q') => {
                    self.exit = true;
                }
                crossterm::event::KeyCode::Char('c') => {
                    // Cycle through colors
                    self.progress_bar_color = match self.progress_bar_color {
                        Color::Green => Color::Yellow,
                        Color::Yellow => Color::LightBlue,
                        Color::LightBlue => Color::LightRed,
                        _ => Color::Green,
                    };
                }
                crossterm::event::KeyCode::Tab => {
                    // Toggle between tabs
                    self.active_tab = (self.active_tab + 1) % 2;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// ====== MAIN FUNCTION ======

fn main() -> io::Result<()> {
    // Initialize terminal
    let mut terminal = ratatui::init();

    // Load data from JSON file
    let file = std::fs::File::open("./data/db.json")?;
    let data: ActivityData = serde_json::from_reader(file)?;

    // Create app instance
    let mut app = App::new(data);

    // Set up event channels
    let (event_tx, event_rx) = mpsc::channel::<Event>();

    // Spawn input handling thread
    let tx_for_input = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_for_input);
    });

    // Spawn background processing thread
    let tx_for_background = event_tx.clone();
    thread::spawn(move || {
        run_background_thread(tx_for_background);
    });

    // Run the app
    let app_result = app.run(&mut terminal, event_rx);

    // Restore terminal
    ratatui::restore();

    app_result
}
