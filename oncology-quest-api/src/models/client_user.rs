use super::user::User;
use crate::utilities::parsable::{Username, Name};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClientUser {
    pub id: i32,
    pub username: Username,
    pub name: Name,
    pub is_admin: bool,
    pub login_count: i32
}

impl From<User> for ClientUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            username: user.username().to_owned(),
            name: user.name().to_owned(),
            is_admin: user.is_admin(),
            login_count: user.login_count()
        }
    }
}