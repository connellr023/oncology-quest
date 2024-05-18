use super::{model::Model, tasks::{Task, SuperTask}};
use crate::utilities::parsables::{EntryIndex, EntryTitle};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use redis::Connection;

const TASKS_KEY: &str = "tasks";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStructure {
    entries: Vec<SuperTask>,
    last_updated: DateTime<Utc>
}

impl TaskStructure {
    /// Updates the current timestamp of when the structure was last updated.
    fn update_timestamp(&mut self) {
        self.last_updated = Utc::now();
    }

    /// Returns the a timestamp of the last update to the structure.
    /// Used for caching purposes.
    pub fn last_updated(&self) -> &DateTime<Utc> {
        &self.last_updated
    }

    /// Updates a single task entry in the structure.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the update operation.
    /// * `index` - The index of the task to update. Slice that can only range from 1 to 3.
    /// * `title` - The new title of the task.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the update operation was successful.
    pub fn update_existing(&mut self, connection: &mut Connection, index: &EntryIndex, title: EntryTitle) -> bool {
        match index.len() {
            1 => self.entries[index.supertask_entry_index()].title = title,
            2 => self.entries[index.supertask_entry_index()].tasks[index.task_entry_index()].title = title,
            3 => self.entries[index.supertask_entry_index()].tasks[index.task_entry_index()].tasks[index.subtask_entry_index()] = title,
            _ => return false
        };

        self.update_timestamp();
        self.update(connection)
    }

    /// Adds a new entry to the structure based on a slice of indicies.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the update operation.
    /// * `index` - The index of the task to add. Expecting the first index to be 0 since supertasks are pushed to the root level.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the addition operation was successful.
    pub fn push_entry(&mut self, connection: &mut Connection, index: &EntryIndex, title: EntryTitle) -> bool {
        match index.len() {
            1 => self.entries.push(SuperTask::new(title)),
            2 => self.entries[index.task_entry_index()].tasks.push(Task::new(title)),
            3 => self.entries[index.task_entry_index()].tasks[index.subtask_entry_index()].tasks.push(title),
            _ => return false
        };

        self.update_timestamp();
        self.update(connection)
    }

    /// Removes the last entry from the structure based on a slice of indicies.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the update operation.
    /// * `index` - The index of the task to remove. Expecting the first index to be 0 since supertasks are popped from the root level.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the removal operation was successful.
    pub fn pop_entry(&mut self, connection: &mut Connection, index: &EntryIndex) -> bool {
        match index.len() {
            1 => {
                self.entries.pop();
            },
            2 => {
                self.entries[index.task_entry_index()].tasks.pop();
            },
            3 => {
                self.entries[index.task_entry_index()].tasks[index.subtask_entry_index()].tasks.pop();
            },
            _ => return false
        };

        self.update_timestamp();
        self.update(connection)
    }
}

impl Model for TaskStructure {
    fn fmt_key(_: &str) -> String {
        TASKS_KEY.to_string()
    }
    
    fn key(&self) -> String {
        TASKS_KEY.to_string()
    }
}