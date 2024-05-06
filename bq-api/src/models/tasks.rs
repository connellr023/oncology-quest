use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type UserTaskEntries = HashMap<u16, HashMap<u16, HashMap<u16, UserTask>>>;

#[derive(Serialize, Deserialize)]
pub struct SubTask {
    pub title: String,
    pub tasks: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub tasks: Vec<SubTask>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserTask {
    pub completed: bool,
    pub comment: String
}