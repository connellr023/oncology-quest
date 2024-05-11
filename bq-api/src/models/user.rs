use super::{model::Model, tasks::UserTaskEntries};
use crate::utilities::parsables::{Parsable, Username, Name, Email, PlainTextPassword};
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use redis::Connection;
use std::collections::HashMap;

pub const USER_KEY_SET: &str = "user_keys";

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: Username,
    pub name: Name,
    pub email: Email,
    pub can_reset_password: bool,
    pub is_admin: bool,
    pub tasks: UserTaskEntries,
    pub salt: u64,
    pub password: String
}

impl Model for User {
    /// Overridden method to store a user in Redis.
    fn store(&self, connection: &mut Connection) -> bool {
        let serialized = match serde_json::to_string(self) {
            Ok(serialized) => serialized,
            Err(_) => return false
        };

        if self.exists(connection) {
            return false;
        }

        let key = self.key();

        let result_pipe = redis::pipe()
            .cmd("SET").arg(&key).arg(serialized).ignore()
            .cmd("SADD").arg(USER_KEY_SET).arg(&key).ignore()
            .query::<()>(connection);

        result_pipe.is_ok()
    }

    fn fmt_key(identifier: &str) -> String {
        format!("user:{}", identifier)
    }

    fn key(&self) -> String {
        Self::fmt_key(self.username.as_str())
    }
}

impl User {
    /// Generates a password hash using the provided salt and password.
    /// 
    /// # Arguments
    /// 
    /// * `salt` - The salt to use when hashing the password.
    /// * `password` - The password to hash.
    /// 
    /// # Returns
    /// 
    /// Returns the hashed password if successful, `None` otherwise.
    pub fn gen_password_hash(salt: u64, password: &str) -> Option<String> {
        match bcrypt::hash(format!("{}{}", password, salt), bcrypt::DEFAULT_COST) {
            Ok(hash) => Some(hash),
            Err(_) => None
        }
    }

    /// Creates a new User instance with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `name` - The name of the user.
    /// * `email` - The email of the user.
    /// * `plain_text_password` - The plain text password of the user.
    /// * `is_admin` - A flag indicating whether the user is an admin or not.
    ///
    /// # Returns
    ///
    /// Returns a new User instance if the password was successfully hashed, `None` otherwise.
    pub fn new(username: Username, name: Name, email: Email, plain_text_password: PlainTextPassword, is_admin: bool) -> Option<Self> {
        let salt = thread_rng().gen::<u64>();
        let password = match Self::gen_password_hash(salt, plain_text_password.as_str()) {
            Some(password) => password,
            None => return None
        };

        Some(Self {
            username,
            name,
            email,
            password,
            salt,
            is_admin,
            can_reset_password: false,
            tasks: HashMap::new()
        })
    }

    /// Validates the provided plain text password against the user's hashed password.
    ///
    /// # Arguments
    ///
    /// * `plain_text_password` - The plain text password to validate.
    ///
    /// # Returns
    ///
    /// Returns `true` if the provided password matches the user's hashed password, `false` otherwise.
    pub fn validate_password(&self, plain_text_password: &str) -> bool {
        bcrypt::verify(format!("{}{}", plain_text_password, self.salt), &self.password).unwrap_or(false)
    }

    /// Gets a reference to the user's tasks.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the user's tasks.
    pub fn tasks_mut(&mut self) -> &mut UserTaskEntries {
        &mut self.tasks
    }

    /// Checks if a given user is an admin.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use.
    /// * `username` - The username of the user to check.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the user is an admin, `false` otherwise.
    pub fn validate_is_admin(connection: &mut Connection, username: &str) -> bool {
        match Self::fetch(connection, username) {
            Some(user) => user.is_admin,
            None => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::client_user::ClientUser;

    #[test]
    fn test_new_user() {
        let username = Username::parse("test-user".to_string()).unwrap();
        let name = Name::parse("Test User".to_string()).unwrap();
        let email = Email::parse("lol@test.com".to_string()).unwrap();
        let password = PlainTextPassword::parse("password".to_string()).unwrap();
        let is_admin = false;

        let user = User::new(username.clone(), name.clone(), email.clone(), password.clone(), is_admin).unwrap();

        assert_eq!(user.username, username);
        assert_eq!(user.name, name);
        assert_ne!(user.password, password.as_str());
        assert_eq!(user.is_admin, is_admin);
    }

    #[test]
    fn test_validate_password() {
        let username = Username::parse("test-user".to_string()).unwrap();
        let name = Name::parse("Test User".to_string()).unwrap();
        let email = Email::parse("lol@test.com".to_string()).unwrap();
        let plain_text_password = PlainTextPassword::parse("password".to_string()).unwrap();

        let user = User::new(username, name, email, plain_text_password.clone(), false).unwrap();

        assert_eq!(user.validate_password(plain_text_password.as_str()), true);
    }

    #[test]
    fn test_from_user_to_client_user() {
        let username = Username::parse("test-user".to_string()).unwrap();
        let name = Name::parse("Test User".to_string()).unwrap();
        let email = Email::parse("lol@test.com".to_string()).unwrap();
        let password = PlainTextPassword::parse("password".to_string()).unwrap();

        let user = User::new(username.clone(), name.clone(), email.clone(), password.clone(), false).unwrap();
        let client_user: ClientUser = user.into();

        assert_eq!(client_user.username, username);
        assert_eq!(client_user.name, name);
        assert_eq!(client_user.email, email);
        assert_eq!(client_user.is_admin, false);
    }
}