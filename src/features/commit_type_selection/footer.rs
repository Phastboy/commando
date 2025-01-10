use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

/// Draws the footer for the commit type selection screen.
pub fn draw_commit_type_footer(frame: &mut Frame, area: Rect) {
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
