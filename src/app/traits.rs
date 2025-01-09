use crossterm::event::KeyEvent;
use std::fmt::Debug;

pub trait Handleable: Debug {
    fn handle_key_event(&mut self, key: KeyEvent);
}
