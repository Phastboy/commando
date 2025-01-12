use crossterm::event::Event;
use ratatui::Frame;
use std::fmt::Debug;

/// Represents a UI component in the application.
pub trait UIComponent: Debug {
    /// Renders the component.
    fn render(&self, context: &mut Frame);

    /// Handles events for the component.
    ///
    /// # Arguments
    /// - `event`: The user event to handle.
    ///
    /// # Returns
    /// - `Ok(true)` if the event was handled; otherwise, `Ok(false)`.
    fn handle_event(&mut self, event: &Event) -> Result<bool, String>;
}
