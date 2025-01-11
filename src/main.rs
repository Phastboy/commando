mod features;
/// The main module for the Commando application.
mod lib;
mod screen_manager;

use features::welcome_screen::WelcomeScreen;
use screen_manager::ScreenManager;

/// The entry point of the application.
fn main() {
    let start_screen = Box::new(WelcomeScreen::new());
    let mut manager = ScreenManager::new(start_screen);

    println!("Starting Commando...");
    manager.run();
    println!("Goodbye!");
}
