use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    widgets::{Block, Borders},
};

pub type DrawFn = fn(&mut Frame, Rect);

pub fn draw_ui(frame: &mut Frame, components: Option<&[(DrawFn, Constraint)]>) {
    if let Some(components) = components {
        let constraints: Vec<Constraint> = components
            .iter()
            .map(|(_, constraint)| *constraint)
            .collect();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints::<&[Constraint]>(constraints.as_ref())
            .split(frame.area());

        for (i, (draw_fn, _)) in components.iter().enumerate() {
            draw_fn(frame, layout[i]);
        }
    } else {
        // Draw a blank page
        frame.render_widget(Block::default().borders(Borders::NONE), frame.area());
    }
}
