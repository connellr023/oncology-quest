use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type UserTaskEntries = HashMap<u16, HashMap<u16, HashMap<u16, UserTask>>>;

#[derive(Serialize, Deserialize)]
pub struct SubTask {
    title: String,
    tasks: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    title: String,
    tasks: Vec<SubTask>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserTask {
    pub completed: bool,
    pub comment: String
}

// #[derive(Serialize, Deserialize)]
// pub struct Tasks {
//     entries: NamedTaskEntries
// }

// #[allow(dead_code)]
// impl Tasks {
//     /// Creates a new `Tasks` instance.
//     pub fn new() -> Self {
//         Self { entries: HashMap::new() }
//     }

//     /// Returns the entries of the tasks.
//     ///
//     /// # Returns
//     ///
//     /// A reference to the map of task entries.
//     pub fn entries(&self) -> &TaskEntries<String, Vec<String>> {
//         &self.entries
//     }
// }

// // TODO: Implement the `Model` trait for `Tasks`.
// // Will only be required for editing by admins