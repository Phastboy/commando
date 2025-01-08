pub use app::App;

pub mod app;
pub mod features;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    // Display the splash screen
    app::splash::show_splash_screen(&mut terminal)?;

    // Start the main application
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
