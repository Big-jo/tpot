
use std::{io, sync::mpsc};


use ratatui::buffer::Buffer;
use ratatui::layout::*;
use ratatui::style::*;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::widgets::{Block, Borders, Widget};
use ratatui::{DefaultTerminal, Frame};

use crate::models::activity::ActivityProgressBar;
use crate::models::task_data::TaskData;
use crate::ActivityData;
use crate::events::event::Event;



pub struct App {
    exit: bool,
    progress_bar_color: Color,
    background_progress: f64,
    data: ActivityData,
    active_tab: usize,
}

impl App {
    pub fn new(data: ActivityData) -> Self {
        App {
            exit: false,
            progress_bar_color: Color::Green,
            background_progress: 0_f64,
            data,
            active_tab: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
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
            .title_style(Style::default().fg(Color::Green))
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
            .title_style(Style::default().fg(Color::LightYellow))
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
                // let mut count: usize = 0;
                TaskData {
                    task_name: task.task_name.clone(),
                    progress: task.progress,
                    timer: task.timer,
                }
                .render(*area, buf);

                // count += 1;
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
