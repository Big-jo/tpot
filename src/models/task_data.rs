use ratatui::{buffer::Buffer, layout::*, style::*, text::Line, widgets::*};

#[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
pub struct TaskData {
    pub task_name: String,
    pub progress: f64,
    pub timer: f64,
    // id: i3
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
