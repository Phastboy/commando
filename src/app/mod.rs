//! The `app` module contains the main application structure and logic.
//!
//! It manages screens, state, and the main event loop.

use crate::screen::manager::ScreenManager;
use crate::screen::Screen;
use crate::state::State;
use crate::ui::render::UIRenderer;
use color_eyre::Result;
use ratatui::DefaultTerminal;

/// The `App` struct manages the application's screens and state.
///
/// # Generics
/// - `S`: A type implementing the `State` trait.
#[derive(Debug, Default)]
pub struct App<S: State> {
    screen_manager: ScreenManager<S>,
}

impl<S: State> App<S> {
    /// Creates a new `App`.
    ///
    /// # Arguments
    /// - `start_screen`: The initial screen of the application.
    /// - `state`: The application state.
    pub fn new(start_screen: Box<dyn Screen>, state: S) -> Self {
        Self {
            screen_manager: ScreenManager::new(start_screen, state),
        }
    }

    /// Runs the main application loop.
    ///
    /// # Arguments
    /// - `interface`: The terminal interface.
    /// - `render_ui`: The UI renderer.
    ///
    /// # Errors
    /// Returns an error if the event loop encounters a problem.
    pub fn run(&mut self, mut interface: DefaultTerminal, mut render_ui: UIRenderer) -> Result<()> {
        loop {
            self.screen_manager.render(&mut interface, &render_ui)?;
            let event = crossterm::event::read()?;
            if let Ok(None) = self.screen_manager.handle_event(event, &mut render_ui) {
                break;
            }
        }
        Ok(())
    }
}
