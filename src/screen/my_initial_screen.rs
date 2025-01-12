//! The initial screen of the application.

use crate::components::text_block::TextBlock;
use crate::screen::Screen;
use crate::state::State;
use crate::ui::render::UIRenderer;
use crossterm::event::Event;
use ratatui::{layout::Rect, DefaultTerminal};
use std::io;

/// Represents the initial screen of the application.
#[derive(Debug, Default)]
pub struct MyInitialScreen;

impl MyInitialScreen {
    /// Creates a new `MyInitialScreen`.
    pub fn new() -> Self {
        Self
    }
}

impl Screen for MyInitialScreen {
    /// Sets up the UI for the initial screen.
    fn setup_ui(&mut self) -> UIRenderer {
        let mut render_ui = UIRenderer::new();
        render_ui.add_component(Box::new(TextBlock::new(
            "Welcome to Commando!".to_string(),
            Rect::new(0, 0, 30, 5),
            true,
        )));
        render_ui
    }

    /// Renders the initial screen.
    fn render(&self, terminal: &mut DefaultTerminal, render_ui: &UIRenderer) -> io::Result<()> {
        terminal.draw(|frame| {
            render_ui.render(frame);
        })?;
        Ok(())
    }

    /// Handles events for the initial screen.
    fn handle_event(
        &mut self,
        event: Event,
        _state: &mut dyn State,
        render_ui: &mut UIRenderer,
    ) -> Result<Option<Box<dyn Screen>>, String> {
        render_ui.handle_event(&event)?;
        Ok(None)
    }
}
