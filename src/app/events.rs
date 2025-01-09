use crate::app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};

pub fn handle(app: &mut App) -> Result<()> {
    match event::read()? {
        // it's important to check KeyEventKind::Press to avoid handling key release events
        Event::Key(key) if key.kind == KeyEventKind::Press => app.on_key_event(key),
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
        _ => {}
    }
    Ok(())
}
