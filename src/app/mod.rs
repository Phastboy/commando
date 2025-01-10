pub mod events;
pub mod traits;

use crate::app::traits::Handleable;
use crate::features::welcome_screen::events::WelcomeScreenHandler;
use color_eyre::Result;
use crossterm::event::Event;
use crossterm::event::{KeyCode};
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
            active_handler: Box::new(WelcomeScreenHandler {}),
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
        println!("Application is quitting...");
        self.running = false;
    }

    pub fn on_event(&mut self, event: Event) {
        self.active_handler.handle_event(&event);
        if let Event::Key(key) = event {
            if let KeyCode::Char('q') = key.code {
                println!("Quit event received. Exiting...");
                self.quit();
            }
        }
    }
}
