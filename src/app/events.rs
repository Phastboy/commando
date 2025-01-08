use crate::app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle(app: &mut App) -> Result<()> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => handle_key(app, key),
        Event::Resize(_, _) => {} // Handle resize if needed
        _ => {}
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Up) => {
            app.selected = if app.selected == 0 {
                app.commit_types.len() - 1
            } else {
                app.selected - 1
            };
        }
        (_, KeyCode::Down) => {
            app.selected = if app.selected == app.commit_types.len() - 1 {
                0
            } else {
                app.selected + 1
            };
        }
        (_, KeyCode::Enter) => {
            println!(
                "Selected Commit Type: {}",
                app.commit_types[app.selected].as_str()
            );
            app.quit(); // Quit after selection for now
        }
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
        _ => {}
    }
}
