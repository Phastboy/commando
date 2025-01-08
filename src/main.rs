use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Create a simple block widget with a border
            let block = Block::default()
                .title(Span::styled(
                    "Commando",
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        // Exit the app when the user presses 'q'
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('a') => println!("Staging files..."),
                KeyCode::Char('c') => println!("Committing changes..."),
                _ => {}
            }
        }
    }
}
