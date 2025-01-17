use crate::features::welcome_screen::{
    content::draw_welcome_content, footer::draw_welcome_footer, header::draw_welcome_header,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Frame;

/// Draws the welcome screen UI.
pub fn draw_welcome_ui(frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .split(area);

    draw_welcome_header(frame, chunks[0]);
    draw_welcome_content(frame, chunks[1]);
    draw_welcome_footer(frame, chunks[2]);
}
