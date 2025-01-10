use crossterm::event::Event;
use std::fmt::Debug;

pub trait Handleable: Debug {
    fn handle_event(&mut self, event: &Event);
}
