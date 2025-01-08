pub mod events;
pub mod state;
pub mod ui;

use crate::features::CommitType;
use color_eyre::Result;
use ratatui::DefaultTerminal;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub commit_types: Vec<CommitType>,
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            commit_types: CommitType::all(),
            selected: 0,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| ui::draw(frame, &self))?;
            events::handle(&mut self)?;
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
