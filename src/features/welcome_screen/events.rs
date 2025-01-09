use crate::app::traits::Handleable;
use crossterm::event::{KeyCode, KeyEvent};
use std::fmt;

#[derive(Default)]
pub struct WelcomeScreenHandler;

impl fmt::Debug for WelcomeScreenHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WelcomeScreenHandler")
            .field("handler", &"WelcomeScreenHandler")
            .finish()
    }
}

impl Handleable for WelcomeScreenHandler {
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                println!("Quit command received from Welcome Screen!");
            }
            _ => println!("Key {:?} pressed in Welcome Screen", key.code),
        }
    }
}
