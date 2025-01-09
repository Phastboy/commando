use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Draws the header for the welcome screen.
pub fn draw_welcome_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new("Welcome to Commando")
        .block(Block::default().borders(Borders::ALL).title("Welcome"))
        .alignment(Alignment::Center);
    frame.render_widget(header, area);
}
