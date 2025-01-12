//! A simple state manager for storing key-value pairs.

use std::collections::HashMap;

/// Manages application state in key-value pairs.
#[derive(Debug, Default)]
pub struct StateManager {
    state: HashMap<String, String>,
}

impl StateManager {
    /// Creates a new `StateManager`.
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    /// Sets a state value by its key.
    pub fn set_state(&mut self, key: &str, value: String) {
        self.state.insert(key.to_string(), value);
    }

    /// Retrieves a state value by its key.
    pub fn get_state(&self, key: &str) -> Option<&String> {
        self.state.get(key)
    }
}
