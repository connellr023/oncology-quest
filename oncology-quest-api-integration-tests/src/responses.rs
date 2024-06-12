use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub is_admin: bool,
    pub login_count: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserTask {
    pub id: i32,
    pub user_id: i32,
    pub subtask_id: i32,
    pub is_completed: bool,
    pub comment: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    id: i32,
    name: String,
    last_updated: DateTime<Utc>
}

#[derive(Deserialize, Debug)]
pub struct SearchResultUser {
    pub user: User,
    pub tasks: HashMap<i32, UserTask>
}

pub type SearchUserResponse = HashMap<i32, SearchResultUser>;

#[derive(Deserialize, Debug)]
pub struct UserSessionResponse {
    pub user: User,
    pub rotations: HashMap<i32, Rotation>,
    pub tasks: Option<HashMap<i32, UserTask>>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRotationResponse {
    pub rotation_id: i32,
    pub last_updated: DateTime<Utc>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Supertask {
    id: i32,
    title: String,
    rotation_id: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: i32,
    supertask_id: i32,
    title: String,
    rotation_id: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subtask {
    id: i32,
    task_id: i32,
    title: String,
    rotation_id: i32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntryLevel<T: Sized, U: Sized> {
    entry: T,
    children: Box<[U]>
}

type EntryHierarchy = EntryLevel<Supertask, EntryLevel<Task, Subtask>>;

#[derive(Deserialize, Debug)]
pub struct EntryStructure(pub Vec<EntryHierarchy>);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntryResponse {
    pub entry_id: i32
}

#[derive(Deserialize)]
pub struct CreateUserTaskResponse {
    pub id: i32
}