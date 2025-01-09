pub mod events;

use crate::utils::ui::draw_ui;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

/// Main application structure.
#[derive(Debug)]
pub struct App {
    /// Indicates whether the application is running.
    pub running: bool,
}

impl App {
    /// Creates a new instance of the application.
    pub fn new() -> Self {
        Self { running: false }
    }

    /// Runs the application.
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| {
                draw_ui(frame, frame.area(), None);
            })?;
            events::handle(self)?;
        }
        Ok(())
    }

    /// Quits the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Handles key events.
    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }
}
