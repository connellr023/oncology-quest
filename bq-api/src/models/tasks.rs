use crate::utilities::parsables::{EntryTitle, Comment};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the structure of user tasks corresponding to their JSON representation.
pub type UserTaskEntries = HashMap<usize, HashMap<usize, HashMap<usize, SubTask>>>;

#[derive(Serialize, Deserialize)]
pub struct SuperTask {
    pub title: EntryTitle,
    pub tasks: Vec<Task>
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: EntryTitle,
    pub tasks: Vec<EntryTitle>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SubTask {
    pub completed: bool,
    pub comment: Comment
}

impl SuperTask {
    pub fn new(title: EntryTitle) -> Self {
        SuperTask {
            title,
            tasks: vec![]
        }
    }
}

impl Task {
    pub fn new(title: EntryTitle) -> Self {
        Task {
            title,
            tasks: vec![]
        }
    }
}