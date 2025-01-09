pub mod events;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::DefaultTerminal;
use crate::features::welcome_screen::ui::draw_welcome_ui;

#[derive(Debug)]
pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: false }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| {
                draw_welcome_ui(frame, frame.area());
            })?;
            events::handle(self)?;
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }
}
