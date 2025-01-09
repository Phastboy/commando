pub mod events;
pub mod traits;

use crate::app::traits::Handleable;
use crate::features::welcome_screen::events::WelcomeScreenHandler;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub active_handler: Box<dyn Handleable>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            active_handler: Box::new(WelcomeScreenHandler::default()),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| {
                crate::features::welcome_screen::ui::draw_welcome_ui(frame, frame.area());
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
            KeyCode::Char('q') => {
                println!("Quit event received. Exiting...");
                self.quit();
            }
            _ => {}
        }
    }
}
