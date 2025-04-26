use ratatui::{buffer::Buffer, layout::*, text::Line, widgets::*, style::*};


pub struct ActivityProgressBar {
    pub progress: f64,
    pub timer: f64,
    pub color: Color,
    pub user_name: String,
    pub task_name: String,
}


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
