use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};

/// Draws the content for the commit type selection screen.
pub fn draw_commit_type_content(frame: &mut Frame, area: Rect) {
    let commit_types = vec![
        "feat: A new feature",
        "fix: A bug fix",
        "chore: Maintenance or non-functional changes",
        "docs: Documentation only changes",
        "style: Changes that do not affect the meaning of the code",
        "refactor: A code change that neither fixes a bug nor adds a feature",
        "perf: A code change that improves performance",
        "test: Adding missing tests or correcting existing tests",
    ];

    let items: Vec<ListItem> = commit_types
        .iter()
        .map(|&item| ListItem::new(item).style(Style::default().fg(Color::White)))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Select Commit Type")
                .style(Style::default().fg(Color::Magenta).bg(Color::Black)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(list, area);
}
