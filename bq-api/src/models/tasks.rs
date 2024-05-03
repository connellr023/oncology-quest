use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Data structure represents all possible task entries.
/// 
/// # Type Parameters
/// 
/// * `U` - The type of the keys in the subtasks.
/// * `V` - The type representing the task.
pub type TaskEntries<U, V> = HashMap<U, HashMap<U, V>>;
pub type NamedTaskEntries = TaskEntries<String, Vec<String>>;
pub type UserTaskEntries = TaskEntries<u16, HashMap<u16, UserTask>>;

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    entries: NamedTaskEntries
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserTask {
    pub completed: bool,
    pub comment: String
}

// For now
#[allow(dead_code)]
impl Tasks {
    /// Creates a new `Tasks` instance.
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    /// Returns the entries of the tasks.
    ///
    /// # Returns
    ///
    /// A reference to the map of task entries.
    pub fn entries(&self) -> &TaskEntries<String, Vec<String>> {
        &self.entries
    }
}

// TODO: Implement the `Model` trait for `Tasks`.
// Will only be required for editing by admins