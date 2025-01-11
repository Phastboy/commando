use commando::Screen;

/// The screen for selecting a commit type.
pub struct SelectCommitTypeScreen;

impl SelectCommitTypeScreen {
    /// Create a new `SelectCommitTypeScreen`.
    pub fn new() -> Self {
        Self
    }
}

impl Screen for SelectCommitTypeScreen {
    /// Render the 'Select Commit Type' screen.
    fn render(&self) {
        println!("This is the 'Select Commit Type' screen. (Coming soon)");
    }

    /// Handle an event. Currently, there is no navigation for this screen.
    fn handle_event(&mut self, _event: String) -> Option<Box<dyn Screen>> {
        None
    }
}
