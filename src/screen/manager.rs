//! Manages transitions and interactions between different screens in the application.

use crate::screen::Screen;
use crate::state::State;
use crate::ui::render::UIRenderer;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use std::io;

/// Handles transitions and events for the current screen.
///
/// # Generics
/// - `S`: A type implementing the `State` trait.
#[derive(Debug)]
pub struct ScreenManager<S: State> {
    current: Box<dyn Screen>,
    state: S,
}

impl<S: State + Default> Default for ScreenManager<S> {
    /// Creates a default `ScreenManager` with the initial screen and state.
    fn default() -> Self {
        Self {
            current: Box::new(crate::screen::my_initial_screen::MyInitialScreen::default()),
            state: S::default(),
        }
    }
}

impl<S: State> ScreenManager<S> {
    /// Creates a new `ScreenManager`.
    ///
    /// # Arguments
    /// - `start_screen`: The initial screen.
    /// - `state`: The state for managing application data.
    pub fn new(start_screen: Box<dyn Screen>, state: S) -> Self {
        let mut state = state;
        state.set_active(true);
        Self {
            current: start_screen,
            state,
        }
    }

    /// Transitions to a new screen.
    ///
    /// # Arguments
    /// - `next_screen`: The new screen to transition to.
    pub fn transition_to(&mut self, next_screen: Box<dyn Screen>) {
        self.state.set_active(false);
        self.current = next_screen;
        self.state.set_active(true);
    }

    /// Renders the current screen.
    ///
    /// # Arguments
    /// - `interface`: The terminal interface.
    /// - `render_ui`: The UI renderer.
    ///
    /// # Errors
    /// Returns an error if rendering fails.
    pub fn render(
        &self,
        interface: &mut DefaultTerminal,
        render_ui: &UIRenderer,
    ) -> io::Result<()> {
        self.current.render(interface, render_ui)
    }

    /// Handles an event and delegates it to the current screen.
    ///
    /// # Arguments
    /// - `event`: The event to handle.
    /// - `render_ui`: The UI renderer.
    ///
    /// # Returns
    /// - `Ok(Some(screen))` if the event triggers a screen transition.
    /// - `Ok(None)` if no transition occurs.
    pub fn handle_event(
        &mut self,
        event: Event,
        render_ui: &mut UIRenderer,
    ) -> Result<Option<Box<dyn Screen>>, String> {
        if self.state.is_active() {
            self.current.handle_event(event, &mut self.state, render_ui)
        } else {
            Ok(None)
        }
    }
}
