use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Borders};
use ratatui::style::{Style, Color};

/// Draws the content for the welcome screen with vibrant colors.
pub fn draw_welcome_content(frame: &mut Frame, area: Rect) {
    let content = Paragraph::new(
        "Commando helps you write high-quality commit messages with ease.\n\n\
        Features include:\n\
        - Selection of commit types (feat, fix, chore, etc.)\n\
        - Scope definition\n\
        - Concise and detailed descriptions\n\
        - Message preview and confirmation\n\n\
        Press 'q' to quit or any other key to explore further.\n\n\n\
            NB: All features not active yet! 😅",
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("About Commando")
            .style(Style::default().fg(Color::Magenta).bg(Color::Black)),
    )
    .style(Style::default().fg(Color::White))
    .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(content, area);
}
