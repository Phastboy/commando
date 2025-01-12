//! The entry point for the application.

pub mod app;
pub mod components;
pub mod screen;
pub mod state;
pub mod ui;

use crate::screen::my_initial_screen::MyInitialScreen;
use crate::screen::Screen;
use crate::state::in_memory::InMemoryState;
use app::App;
use color_eyre::eyre::Result;

/// Starts the application.
fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let mut start_screen = Box::new(MyInitialScreen::new());
    let state = InMemoryState::default();
    let render_ui = start_screen.setup_ui();

    App::new(start_screen, state).run(terminal, render_ui)?;

    ratatui::restore();
    Ok(())
}
