use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

/// Draws the header for the commit type selection screen.
pub fn draw_commit_type_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new("Select Commit Type")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Commit Type Selection")
                .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(header, area);
}
