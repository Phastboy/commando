use crate::events;
use crate::ui::{draw_ui, DrawFn}; // Import draw_ui and DrawFn
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Constraint,
    prelude::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
    DefaultTerminal,
};

#[derive(Debug)]
pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: false }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| {
                let components = &[
                    (draw_header as DrawFn, Constraint::Length(3)),
                    (draw_main_section as DrawFn, Constraint::Min(5)),
                    (draw_footer as DrawFn, Constraint::Length(3)),
                ];
                draw_ui(frame, components);
            })?;
            events::handle(self)?;
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }
}

// Define the draw_header, draw_main_section, and draw_footer functions here
fn draw_header(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Block::default().title("Header").borders(Borders::ALL),
        area,
    );
}

fn draw_main_section(frame: &mut Frame, area: Rect) {
    let _section = Paragraph::new("Main content goes here...")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);
    frame.render_widget(
        Block::default().title("Section").borders(Borders::ALL),
        area,
    );
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Block::default().title("Footer").borders(Borders::ALL),
        area,
    );
}
