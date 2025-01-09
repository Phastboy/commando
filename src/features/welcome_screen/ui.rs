use ratatui::prelude::Frame;
use ratatui::layout::{Layout, Constraint, Direction, Rect};
use crate::features::welcome_screen::header::draw_welcome_header;

/// Draws the welcome screen UI.
pub fn draw_welcome_ui(frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ].as_ref(),
        )
        .split(area);

    draw_welcome_header(frame, chunks[0]);
}
