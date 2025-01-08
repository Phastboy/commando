use dialoguer::{Input, Select, theme::ColorfulTheme};

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

    // Prompt for scope (optional)
    let scope: String = Input::with_theme(&theme)
        .with_prompt("Enter the scope (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()
        .expect("Failed to read scope");

    // Print the collected inputs
    println!("\nConventional Commit:");
    println!(
        "{}({})",
        commit_types[commit_type],
        if scope.is_empty() { "none" } else { &scope }
    );
}

