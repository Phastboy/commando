use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fs::write;

pub fn run_interactive() {
    let theme = ColorfulTheme::default();
    let commit_types = vec!["feat", "fix", "chore", "docs", "refactor", "test", "ci"];

    // Gather inputs
    let commit_type = select_commit_type(&commit_types, &theme);
    let scope = prompt_scope(&theme);
    let description = prompt_description(&theme);

    // Generate commit message
    let commit_message = generate_commit_message(&commit_types[commit_type], &scope, &description);

    // Display commit message
    println!("\nGenerated Conventional Commit:\n{}", commit_message);

    // Save commit message to file
    save_commit_message(&commit_message);
}

/// Prompt for commit type
fn select_commit_type(commit_types: &[&str], theme: &ColorfulTheme) -> usize {
    Select::with_theme(theme)
        .with_prompt("Select the type of change")
        .items(commit_types)
        .default(0)
        .interact()
        .expect("Failed to select commit type")
}

/// Prompt for scope (optional)
fn prompt_scope(theme: &ColorfulTheme) -> String {
    Input::with_theme(theme)
        .with_prompt("Enter the scope (optional, press Enter to skip)")
        .allow_empty(true)
        .interact_text()
        .expect("Failed to read scope")
}

/// Prompt for description (required, validate length)
fn prompt_description(theme: &ColorfulTheme) -> String {
    Input::with_theme(theme)
        .with_prompt("Enter the description (max 50 characters)")
        .validate_with(|input: &String| {
            if input.is_empty() {
                Err("Description cannot be empty.")
            } else if input.len() > 50 {
                Err("Description must be 50 characters or less.")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .expect("Failed to read description")
}

/// Generate commit message
fn generate_commit_message(commit_type: &str, scope: &str, description: &str) -> String {
    if scope.is_empty() {
        format!("{}: {}", commit_type, description)
    } else {
        format!("{}({}): {}", commit_type, scope, description)
    }
}

/// Save commit message to a file
fn save_commit_message(commit_message: &str) {
    let path = ".git/COMMIT_EDITMSG";
    if let Err(e) = write(path, commit_message) {
        eprintln!("Failed to save commit message to file: {}", e);
    } else {
        println!("\nCommit message saved to {}", path);
    }
}

