use crate::utilities::parsables::{SubtaskTitle, Comment};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the structure of user tasks corresponding to their JSON representation.
pub type UserTaskEntries = HashMap<usize, HashMap<usize, HashMap<usize, Subtask>>>;

#[derive(Serialize, Deserialize)]
pub struct Supertask {
    pub title: SubtaskTitle,
    pub tasks: Vec<Task>
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: SubtaskTitle,
    pub tasks: Vec<SubtaskTitle>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Subtask {
    pub completed: bool,
    pub comment: Comment
}

impl Supertask {
    pub fn new(title: SubtaskTitle) -> Self {
        Supertask {
            title,
            tasks: vec![]
        }
    }
}

impl Task {
    pub fn new(title: SubtaskTitle) -> Self {
        Task {
            title,
            tasks: vec![]
        }
    }
}