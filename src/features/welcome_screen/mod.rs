use crate::features::commit_type_screen::SelectCommitTypeScreen;
use commando::Screen;

/// The welcome screen of the application.
pub struct WelcomeScreen;

impl WelcomeScreen {
    /// Create a new `WelcomeScreen`.
    pub fn new() -> Self {
        Self
    }
}

impl Screen for WelcomeScreen {
    /// Render the welcome screen.
    fn render(&self) {
        println!("Welcome to Commando!");
        println!("This app helps you write conventional commit messages interactively.");
        println!("Press Enter to proceed to the next step or type 'exit' to quit.");
    }

    /// Handle an event and optionally return the next screen.
    fn handle_event(&mut self, event: String) -> Option<Box<dyn Screen>> {
        match event.as_str() {
            "" => Some(Box::new(SelectCommitTypeScreen::new())), // Proceed to the next screen
            "exit" => None,
            _ => {
                println!("Invalid input. Please press Enter to proceed or type 'exit' to quit.");
                None
            }
        }
    }
}
