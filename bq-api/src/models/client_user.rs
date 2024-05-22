use super::user::User;
use crate::utilities::parsable::{Username, Name, Email};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClientUser {
    id: i32,
    username: Username,
    name: Name,
    email: Email,
    is_admin: bool,
    login_count: i32
}

impl ClientUser {
    pub fn new(id: i32, username: Username, name: Name, email: Email, is_admin: bool, login_count: i32) -> Self {
        Self {
            id,
            username,
            name,
            email,
            is_admin,
            login_count
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn login_count(&self) -> i32 {
        self.login_count
    }
}

impl From<User> for ClientUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            username: *user.username(),
            name: *user.name(),
            email: *user.email(),
            is_admin: user.is_admin(),
            login_count: user.login_count()
        }
    }
}