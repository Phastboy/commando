use crate::features::CommitType;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    running: bool,                 // app is running?
    commit_types: Vec<CommitType>, // Store commit types
    selected: usize,               // Index of the selected commit type
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            commit_types: vec![
                CommitType::Feat,
                CommitType::Fix,
                CommitType::Docs,
                CommitType::Style,
                CommitType::Refactor,
                CommitType::Perf,
                CommitType::Test,
                CommitType::Chore,
                CommitType::CI,
            ],
            selected: 0,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(4), // Header
                    Constraint::Min(5),    // Commit Types List
                    Constraint::Length(3), // Footer
                ]
                .as_ref(),
            )
            .split(frame.area());

        // Header
        let title = Line::from("🚀 Commando")
            .bold()
            .fg(Color::LightBlue)
            .centered();
        let header = Paragraph::new(format!(
            "🌟 Select the type of your commit 🌟\nCurrently Selected: {}",
            self.commit_types[self.selected].as_str()
        ))
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::White).bg(Color::Black));
        frame.render_widget(header, chunks[0]);

        // Commit Types
        let rows: Vec<Constraint> = vec![Constraint::Length(3); self.commit_types.len()];
        let rows_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(rows.as_slice())
            .split(chunks[1]);

        for (i, row) in rows_layout.iter().enumerate() {
            let commit_type = &self.commit_types[i];
            let text = Paragraph::new(format!(
                "→ {} - {}",
                commit_type.as_str(),
                commit_type.description()
            ))
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(if self.selected == i {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Cyan)
                    }),
            );
            frame.render_widget(text, *row);
        }

        // Footer
        let footer = Paragraph::new("↑/↓ to navigate, Enter to select, Esc to quit")
            .style(
                Style::default()
                    .fg(Color::LightGreen)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, chunks[2]);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Up) => {
                self.selected = if self.selected == 0 {
                    self.commit_types.len() - 1
                } else {
                    self.selected - 1
                };
            }
            (_, KeyCode::Down) => {
                self.selected = if self.selected == self.commit_types.len() - 1 {
                    0
                } else {
                    self.selected + 1
                };
            }
            (_, KeyCode::Enter) => {
                println!(
                    "Selected Commit Type: {}",
                    self.commit_types[self.selected].as_str()
                );
                self.quit(); // Quit after selection for now
            }
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
