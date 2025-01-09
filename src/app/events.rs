/// Handles the key and other events for the application.
pub fn handle(app: &mut App) -> Result<()> {
    match event::read()? {
        // It's important to check KeyEventKind::Press to avoid handling key release events.
        Event::Key(key) if key.kind == KeyEventKind::Press => app.on_key_event(key),
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
        _ => {}
    }
    Ok(())
}
