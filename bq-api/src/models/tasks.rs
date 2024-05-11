use crate::utilities::parsables::{EntryTitle, Comment};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the structure of user tasks corresponding to their JSON representation.
pub type UserTaskEntries = HashMap<u16, HashMap<u16, HashMap<u16, UserTask>>>;

#[derive(Serialize, Deserialize)]
pub struct SubTask {
    pub title: EntryTitle,
    pub tasks: Vec<EntryTitle>
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: EntryTitle,
    pub tasks: Vec<SubTask>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserTask {
    pub completed: bool,
    pub comment: Comment
}

impl Task {
    pub fn new(title: EntryTitle) -> Self {
        Task {
            title,
            tasks: vec![]
        }
    }
}

impl SubTask {
    pub fn new(title: EntryTitle) -> Self {
        SubTask {
            title,
            tasks: vec![]
        }
    }
}