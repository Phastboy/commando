use crate::app::App;
use color_eyre::Result;
use crossterm::event;

pub fn handle(app: &mut App) -> Result<()> {
    let evt = event::read()?;
    println!("Processing event: {:?}", evt);
    app.on_event(evt);
    Ok(())
}
