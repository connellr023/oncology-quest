use serde::{Deserialize, Serialize};

/// Represents a task entry. Will only be used for serialization and deserialization for editing by admins.
pub type TaskEntry = HashMap<String, HashMap<String, Vec<String>>>;

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    entries: HashMap<String, TaskEntry>
}

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
    pub fn entries(&self) -> &HashMap<String, TaskEntry> {
        &self.entries
    }
}

// TODO: Implement the `Model` trait for `Tasks`.
// Will only be required for editing by admins