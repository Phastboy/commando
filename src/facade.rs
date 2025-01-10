use crate::app::traits::Handleable;
use crate::features::welcome_screen::events::WelcomeScreenHandler;
use crate::features::welcome_screen::ui::draw_welcome_ui;
use color_eyre::Result;
use crossterm::event::Event;
use crossterm::event::{KeyCode};
use ratatui::DefaultTerminal;

pub struct CommandoFacade {
    app: App,
    terminal: DefaultTerminal,
}

impl CommandoFacade {
    pub fn new() -> Self {
        Self {
            app: App::new(),
            terminal: ratatui::init(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.app.running = true;
        while self.app.running {
            self.terminal.draw(|frame| {
                draw_welcome_ui(frame, frame.area());
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        let evt = crossterm::event::read()?;
        println!("Processing event: {:?}", evt);
        self.app.on_event(evt);
        Ok(())
    }

    pub fn quit(&mut self) {
        println!("Application is quitting...");
        self.app.running = false;
    }

    pub fn on_event(&mut self, event: Event) {
        self.app.active_handler.handle_event(&event);
        if let Event::Key(key) = event {
            if let KeyCode::Char('q') = key.code {
                println!("Quit event received. Exiting...");
                self.quit();
            }
        }
    }
}
