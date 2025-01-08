use dialoguer::{Select, theme::ColorfulTheme};

pub fn run_interactive() {
    // theme
    let theme = ColorfulTheme::default();

    // commit types
    let commit_types = vec!["feat", "fix", "chore", "docs", "refactor", "test"];

    // prompt for commit type
    let commit_type = Select::with_theme(&theme)
        .with_prompt("Select the type of change")
        .items(&commit_types)
        .default(0)
        .interact()
        .expect("Failed to select commit type");

    println!("Selected commit type: {}", commit_types[commit_type]);
}
