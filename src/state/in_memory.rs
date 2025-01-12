//! Implements an in-memory state management system.

use crate::state::State;
use std::collections::HashMap;

/// Represents an in-memory state storage.
#[derive(Debug, Default)]
pub struct InMemoryState {
    state: HashMap<String, String>,
    active: bool,
}

impl State for InMemoryState {
    /// Sets a key-value pair in the state.
    fn set(&mut self, key: &str, value: String) {
        self.state.insert(key.to_string(), value);
    }

    /// Retrieves a value by its key.
    fn get(&self, key: &str) -> Option<&String> {
        self.state.get(key)
    }

    /// Clears all state entries.
    fn clear(&mut self) {
        self.state.clear();
    }

    /// Sets the active state.
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Checks if the state is active.
    fn is_active(&self) -> bool {
        self.active
    }
}
