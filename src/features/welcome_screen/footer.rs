use ratatui::widgets::{Block, Paragraph, Borders};
use ratatui::prelude::*;
use ratatui::style::{Style, Color};

/// Draws the footer for the welcome screen with vibrant colors.
pub fn draw_welcome_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("Navigation:\n- 'q': Quit\n- Arrow Keys: Navigate\n- Enter: Select")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .style(Style::default().fg(Color::Green).bg(Color::Black)),
        )
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}
