use std::fmt::Debug;

/// Represents a generic state management system.
pub trait State: Debug {
    /// Sets a key-value pair in the state.
    fn set(&mut self, key: &str, value: String);

    /// Retrieves a value by its key.
    fn get(&self, key: &str) -> Option<&String>;

    /// Clears all state entries.
    fn clear(&mut self);

    /// Sets the active state.
    fn set_active(&mut self, active: bool);

    /// Checks if the state is active.
    fn is_active(&self) -> bool;
}
