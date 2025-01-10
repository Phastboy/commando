use crate::features::commit_type_selection::{
    content::draw_commit_type_content, footer::draw_commit_type_footer, header::draw_commit_type_header,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Frame;

/// Draws the commit type selection screen UI.
pub fn draw_commit_type_ui(frame: &mut Frame, area: Rect) {
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

    draw_commit_type_header(frame, chunks[0]);
    draw_commit_type_content(frame, chunks[1]);
    draw_commit_type_footer(frame, chunks[2]);
}
