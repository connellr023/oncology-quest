use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserTask {
    index: (u16, u16, u16),
    completed: bool,
    comment: String
}

/// Represents a task.
impl UserTask {
    /// Creates a new task with the given index, completion status, and comment.
    ///
    /// # Arguments
    ///
    /// * `index` - A tuple representing the index of the task in the task strucure.
    /// * `completed` - A boolean indicating whether the task is completed or not.
    /// * `comment` - A string containing additional comments about the task.
    ///
    /// # Returns
    ///
    /// A new `Task` instance.
    pub fn new(index: (u16, u16, u16), completed: bool, comment: String) -> Self {
        Self {
            index,
            completed,
            comment
        }
    }

    /// Returns the index of the task.
    ///
    /// # Returns
    ///
    /// A tuple representing the index of the task.
    pub fn index(&self) -> (u16, u16, u16) {
        self.index
    }

    /// Returns the completion status of the task.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the task is completed or not.
    pub fn completed(&self) -> bool {
        self.completed
    }

    /// Returns the comment associated with the task.
    ///
    /// # Returns
    ///
    /// A reference to the comment string.
    pub fn comment(&self) -> &str {
        &self.comment
    }
}