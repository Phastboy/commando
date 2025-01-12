//! Defines the `TextBlock` component for displaying text in the UI.

use crate::ui::UIComponent;
use crossterm::event::Event;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

/// Represents a block of text displayed in the UI.
#[derive(Debug, Default)]
pub struct TextBlock {
    text: String,
    area: Rect,
    active: bool,
}

impl TextBlock {
    /// Creates a new `TextBlock`.
    ///
    /// # Arguments
    /// - `text`: The text to display.
    /// - `area`: The layout area for the block.
    /// - `active`: Whether the block is active.
    pub fn new(text: String, area: Rect, active: bool) -> Self {
        Self { text, area, active }
    }

    /// Sets the active state of the block.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Checks if the block is active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl UIComponent for TextBlock {
    /// Renders the `TextBlock`.
    ///
    /// # Arguments
    /// - `context`: The rendering context.
    fn render(&self, context: &mut Frame) {
        if !self.active {
            return; // Skip rendering if inactive
        }

        let block = Block::default().title(&*self.text).borders(Borders::ALL);
        context.render_widget(block, self.area);
    }

    /// Handles events for the `TextBlock`.
    ///
    /// # Arguments
    /// - `event`: The user event to handle.
    ///
    /// # Returns
    /// `Ok(true)` if the event was handled; otherwise, `Ok(false)`.
    fn handle_event(&mut self, event: &Event) -> Result<bool, String> {
        if !self.active {
            return Ok(false);
        }

        if let Event::Key(key_event) = event {
            if key_event.code == crossterm::event::KeyCode::Enter {
                println!("Enter key pressed inside TextBlock");
                return Ok(true);
            }
        }
        Ok(false)
    }
}
