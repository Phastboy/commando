/// A screen in the application.
pub trait Screen {
    /// Render the screen.
    fn render(&self);

    /// Handle an event and optionally return the next screen.
    fn handle_event(&mut self, event: String) -> Option<Box<dyn Screen>>;
}
