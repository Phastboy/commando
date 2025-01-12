//! The `screen` module contains traits and implementations for individual screens.

pub mod manager;
pub mod my_initial_screen;

use crate::state::State;
use crate::ui::render::UIRenderer;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use std::fmt::Debug;
use std::io;

/// Represents a screen in the application.
pub trait Screen: Debug {
    /// Sets up the initial UI for the screen.
    fn setup_ui(&mut self) -> UIRenderer;

    /// Renders the screen.
    ///
    /// # Arguments
    /// - `terminal`: The terminal interface.
    /// - `render_ui`: The UI renderer.
    ///
    /// # Errors
    /// Returns an error if rendering fails.
    fn render(&self, terminal: &mut DefaultTerminal, render_ui: &UIRenderer) -> io::Result<()>;

    /// Handles an event and processes it for the screen.
    ///
    /// # Arguments
    /// - `event`: The user event.
    /// - `state`: The application state.
    /// - `render_ui`: The UI renderer.
    ///
    /// # Returns
    /// - `Ok(Some(next_screen))` if the event triggers a screen transition.
    /// - `Ok(None)` if no transition occurs.
    fn handle_event(
        &mut self,
        event: Event,
        state: &mut dyn State,
        render_ui: &mut UIRenderer,
    ) -> Result<Option<Box<dyn Screen>>, String>;
}
