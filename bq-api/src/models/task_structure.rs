use super::{tasks::Task, model::Model};
use serde::{Deserialize, Serialize};

const TASKS_KEY: &str = "tasks";

#[derive(Serialize, Deserialize)]
pub struct TaskStructure(Vec<Task>);

impl TaskStructure {
    /// Creates a new `TaskStructure` instance.
    pub fn new(entries: Vec<Task>) -> Self {
        Self(entries)
    }

    /// Returns the owned structure of tasks and consumes self.
    pub fn structure_as_owned(self) -> Vec<Task> {
        self.0
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