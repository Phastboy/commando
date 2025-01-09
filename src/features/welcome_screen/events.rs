use crate::app::traits::Handleable;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Default, Debug)]
pub struct WelcomeScreenHandler;

impl Handleable for WelcomeScreenHandler {
    fn handle_key_event(&mut self, key: KeyEvent) {
        if let KeyCode::Char('q') = key.code {
            println!("Quit command received from Welcome Screen!");
        } else {
            println!("Key {:?} pressed in Welcome Screen", key.code);
        }
    }
}
