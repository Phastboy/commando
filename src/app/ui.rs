use crate::features::CommitType;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Stylize},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &crate::app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(5),    // Commit Types List
                Constraint::Length(3), // Footer
            ]
            .as_ref(),
        )
        .split(frame.area());

    draw_header(frame, chunks[0], &app.commit_types[app.selected]);
    draw_commit_types(frame, chunks[1], app);
    draw_footer(frame, chunks[2]);
}

fn draw_header(frame: &mut Frame, area: ratatui::prelude::Rect, selected: &CommitType) {
    let header = Paragraph::new(format!(
        "Select the type of your commit\nCurrently Selected: {}",
        selected.as_str()
    ))
    .style(Style::default().fg(Color::LightBlue).bold())
    .alignment(Alignment::Center);
    frame.render_widget(header, area);
}

fn draw_commit_types(frame: &mut Frame, area: ratatui::prelude::Rect, app: &crate::app::App) {
    let rows: Vec<Constraint> = vec![Constraint::Length(3); app.commit_types.len()];
    let rows_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(rows.as_slice())
        .split(area);

    for (i, row) in rows_layout.iter().enumerate() {
        let commit_type = &app.commit_types[i];
        let description = if app.selected == i {
            Text::styled(
                format!("{} - {}", commit_type.as_str(), commit_type.description()),
                Style::default()
                    .fg(Color::LightGreen)
                    .bold()
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Text::styled(commit_type.as_str(), Style::default().fg(Color::White))
        };

        let text = Paragraph::new(description)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(if app.selected == i {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }),
            )
            .alignment(Alignment::Left);
        frame.render_widget(text, *row);
    }
}

fn draw_footer(frame: &mut Frame, area: ratatui::prelude::Rect) {
    let footer = Paragraph::new("↑/↓ to navigate, Enter to select, Esc to quit")
        .style(
            Style::default()
                .fg(Color::LightGreen)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    frame.render_widget(footer, area);
}
