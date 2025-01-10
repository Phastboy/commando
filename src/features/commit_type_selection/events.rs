use crate::app::traits::Handleable;
use crossterm::event::{Event, KeyCode};

#[derive(Default, Debug)]
pub struct CommitTypeSelectionHandler;

impl Handleable for CommitTypeSelectionHandler {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    println!("Quit command received from Commit Type Selection Screen!");
                }
                _ => {
                    println!("Key {:?} pressed in Commit Type Selection Screen", key.code);
                }
            },
            Event::Mouse(mouse_event) => {
                println!("Mouse event: {:?}", mouse_event);
            }
            Event::Resize(width, height) => {
                println!("Screen resized to width: {}, height: {}", width, height);
            }
            _ => {}
        }
    }
}
