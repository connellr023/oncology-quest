use super::{model::Model, tasks::{SubTask, Task}};
use serde::{Deserialize, Serialize};
use redis::Connection;

const TASKS_KEY: &str = "tasks";

#[derive(Serialize, Deserialize)]
pub struct TaskStructure(Vec<Task>);

impl TaskStructure {
    /// Returns the owned structure of tasks and consumes self.
    pub fn structure_as_owned(self) -> Vec<Task> {
        self.0
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
    pub fn update_existing(&mut self, connection: &mut Connection, index: &[u16], title: &str) -> bool {
        let entries = &mut self.0;
        
        match index.len() {
            1 => entries[index[0] as usize].title = title.to_string(),
            2 => entries[index[0] as usize].tasks[index[1] as usize].title = title.to_string(),
            3 => entries[index[0] as usize].tasks[index[1] as usize].tasks[index[2] as usize] = title.to_string(),
            _ => return false
        };

        self.update(connection)
    }

    /// Adds a new entry to the structure based on a slice of indicies.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the update operation.
    /// * `index` - The index of the task to add. Slice that can only range in length from 0 to 2.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the addition operation was successful.
    pub fn push_entry(&mut self, connection: &mut Connection, index: &[u16], title: &str) -> bool {
        let entries = &mut self.0;

        match index.len() {
            0 => entries.push(Task::new(title)),
            1 => entries[index[0] as usize].tasks.push(SubTask::new(title)),
            2 => entries[index[0] as usize].tasks[index[1] as usize].tasks.push(title.to_string()),
            _ => return false
        };

        self.update(connection)
    }

    /// Removes the last entry from the structure based on a slice of indicies.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the update operation.
    /// * `index` - The index of the task to remove. Slice that can only range in length from 0 to 2.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the removal operation was successful.
    pub fn pop_entry(&mut self, connection: &mut Connection, index: &[u16]) -> bool {
        let entries = &mut self.0;

        match index.len() {
            0 => {
                entries.pop();
            },
            1 => {
                entries[index[0] as usize].tasks.pop();
            },
            2 => {
                entries[index[0] as usize].tasks[index[1] as usize].tasks.pop();
            },
            _ => return false
        };

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