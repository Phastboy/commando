use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    prelude::Frame,
    widgets::{Block, Borders},
};

pub type DrawFn = fn(&mut Frame, Rect);

pub struct UIComponent<'a> {
    pub draw_fn: DrawFn,
    pub constraint: Constraint,
    pub title: Option<&'a str>,
    pub bordered: bool,
}

pub fn draw_ui(frame: &mut Frame, area: Rect, components: Option<&[UIComponent<'_>]>) {
    if let Some(components) = components {
        let constraints: Vec<Constraint> = components.iter().map(|c| c.constraint).collect();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints.as_slice())
            .split(area);

        for (i, component) in components.iter().enumerate() {
            let mut sub_area = layout[i];

            if component.bordered || component.title.is_some() {
                let mut block = Block::default();
                if component.bordered {
                    block = block.borders(Borders::ALL);
                }
                if let Some(title) = component.title {
                    block = block.title(title);
                }
                frame.render_widget(block, sub_area);

                sub_area = sub_area.inner(Margin {
                    vertical: 1,
                    horizontal: 1,
                });
            }

            (component.draw_fn)(frame, sub_area);
        }
    } else {
        frame.render_widget(Block::default().borders(Borders::NONE), area);
    }
}
