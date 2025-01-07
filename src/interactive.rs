use dialoguer::Select;

pub fn run_interactive() {
    // commit types
    let commit_types = vec!["feat", "fix", "chore", "docs", "refactor", "test"];

    // prompt for commit type
    let commit_type = Select::new()
        .with_prompt("Select the type of change")
        .items(&commit_types)
        .default(0)
        .interact()
        .expect("Failed to select commit type");

    println!("Selected commit type: {}", commit_types[commit_type]);
}
