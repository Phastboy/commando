use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Line, Span, Stylize},
    style::{Color, Style},
    widgets::Paragraph,
};
use std::{thread, time::Duration};

pub fn show_splash_screen(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    let commando_text = Line::from(vec![
        Span::styled("COMM", Style::default().fg(Color::Yellow).bold()),
        Span::styled("ANDO 🚀", Style::default().fg(Color::LightBlue).bold()),
    ]);

    let splash_message = Paragraph::new(vec![
        commando_text,
        Line::from(Span::styled(
            "An interactive tool for conventional commits.",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "Press Enter to start!",
            Style::default().fg(Color::LightGreen).bold(),
        )),
    ])
    .alignment(Alignment::Center) // Align the text to the center
    .style(Style::default().fg(Color::White));

    let loading_animation = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut index = 0;
    let start = std::time::Instant::now();

    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(frame.area());

            frame.render_widget(splash_message.clone(), layout[0]);

            let loading_message = Paragraph::new(Line::from(vec![
                Span::styled("Loading ", Style::default().fg(Color::White)),
                Span::styled(
                    loading_animation[index],
                    Style::default().fg(Color::Yellow).bold(),
                ),
            ]))
            .alignment(Alignment::Center) // Align the loading text to the center
            .style(Style::default().fg(Color::White));
            frame.render_widget(loading_message, layout[1]);
        })?;

        // Cycle through the loading animation
        index = (index + 1) % loading_animation.len();
        thread::sleep(Duration::from_millis(100));

        if std::time::Instant::now().duration_since(start) > Duration::from_secs(5) {
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Enter {
                    break;
                }
            }
        }
    }

    Ok(())
}
