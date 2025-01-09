use crate::app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};

pub fn handle(app: &mut App) -> Result<()> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            println!("Processing key event: {:?}", key.code);
            app.on_key_event(key)
        }
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
        _ => {}
    }
    Ok(())
}
