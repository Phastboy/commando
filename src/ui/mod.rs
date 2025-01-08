use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Frame},
};

pub type DrawFn = fn(&mut Frame, Rect); // Ensure this is public

pub fn draw_ui(frame: &mut Frame, components: &[(DrawFn, Constraint)]) {
    let constraints: Vec<Constraint> = components
        .iter()
        .map(|(_, constraint)| *constraint)
        .collect();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints::<&[Constraint]>(constraints.as_ref()) // Specify the generic argument
        .split(frame.area());

    for (i, (draw_fn, _)) in components.iter().enumerate() {
        draw_fn(frame, layout[i]);
    }
}
