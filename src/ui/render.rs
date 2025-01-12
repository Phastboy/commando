//! Handles rendering and event delegation for UI components.

use crate::ui::UIComponent;
use crossterm::event::Event;
use ratatui::Frame;

/// Manages and renders multiple UI components.
#[derive(Debug, Default)]
pub struct UIRenderer {
    components: Vec<Box<dyn UIComponent>>,
}

impl UIRenderer {
    /// Creates a new `UIRenderer`.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Adds a component to the renderer.
    pub fn add_component(&mut self, component: Box<dyn UIComponent>) {
        self.components.push(component);
    }

    /// Removes a component from the renderer.
    ///
    /// # Arguments
    ///
    /// - `index`: The index of the component to remove.
    pub fn remove_component(&mut self, index: usize) {
        if index < self.components.len() {
            self.components.remove(index);
        }
    }

    /// Renders all components.
    ///
    /// # Arguments
    /// - `frame`: The rendering frame.
    pub fn render(&self, frame: &mut Frame) {
        for component in &self.components {
            component.render(frame);
        }
    }

    /// Delegates an event to all components.
    ///
    /// # Arguments
    /// - `event`: The event to handle.
    ///
    /// # Returns
    /// - `Ok(true)` if any component handled the event.
    /// - `Ok(false)` if no component handled the event.
    pub fn handle_event(&mut self, event: &Event) -> Result<bool, String> {
        // Borrow `event`
        for component in &mut self.components {
            if component.handle_event(event)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
