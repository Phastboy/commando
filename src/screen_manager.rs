use commando::Screen;

/// Manages the current screen and application flow.
pub struct ScreenManager {
    current: Box<dyn Screen>,
}

impl ScreenManager {
    /// Create a new `ScreenManager` with the starting screen.
    pub fn new(start_screen: Box<dyn Screen>) -> Self {
        Self {
            current: start_screen,
        }
    }

    /// Run the application, rendering the current screen and handling user input.
    pub fn run(&mut self) {
        loop {
            self.current.render();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_string();

            if let Some(next_screen) = self.current.handle_event(input) {
                self.current = next_screen;
            } else {
                break;
            }
        }
    }
}
