pub mod events;
pub mod traits;

use crate::facade::CommandoFacade;
use color_eyre::Result;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub facade: CommandoFacade,
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
            facade: CommandoFacade::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.running = true;
        self.facade.run()?;
        Ok(())
    }

    pub fn quit(&mut self) {
        println!("Application is quitting...");
        self.running = false;
    }
}
