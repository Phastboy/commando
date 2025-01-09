use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

/// Draws the header for the welcome screen with vibrant colors.
pub fn draw_welcome_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new("🚀 Welcome to Commando 🚀\nYour interactive CLI for crafting conventional commit messages!")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Welcome")
                .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(header, area);
}
