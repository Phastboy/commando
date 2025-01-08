#[derive(Debug, Clone, Copy)]
/// Enum representing different types of commits.
pub enum CommitType {
    Feat,
    Fix,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
    Chore,
    CI,
}

impl CommitType {
    /// Returns the string representation of the commit type.
    pub fn as_str(&self) -> &'static str {
        match self {
            CommitType::Feat => "feat 🌟",
            CommitType::Fix => "fix 🐛",
            CommitType::Docs => "docs 📚",
            CommitType::Style => "style 💄",
            CommitType::Refactor => "refactor 🔨",
            CommitType::Perf => "perf 🚀",
            CommitType::Test => "test ✅",
            CommitType::Chore => "chore 🧹",
            CommitType::CI => "ci ⚙️",
        }
    }

    /// Returns a brief description of the commit type.
    pub fn description(&self) -> &'static str {
        match self {
            CommitType::Feat => "A new feature 🌟",
            CommitType::Fix => "A bug fix 🐛",
            CommitType::Docs => "Documentation only changes 📚",
            CommitType::Style => "Changes that do not affect the code 💄",
            CommitType::Refactor => "Code changes that neither fix bugs nor add features 🔨",
            CommitType::Perf => "Performance improvements 🚀",
            CommitType::Test => "Adding or modifying tests ✅",
            CommitType::Chore => "Other changes that don't modify src or tests 🧹",
            CommitType::CI => "Changes related to continuous integration ⚙️",
        }
    }
}
