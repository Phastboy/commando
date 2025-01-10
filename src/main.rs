pub use app::App;

pub mod app;
pub mod features;
pub mod utils;
pub mod facade;

use facade::CommandoFacade;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut facade = CommandoFacade::new();
    let result = facade.run();
    ratatui::restore();
    result
}
