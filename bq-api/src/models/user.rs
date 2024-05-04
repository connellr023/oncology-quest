use super::{model::Model, tasks::UserTaskEntries};
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use redis::Connection;
use std::collections::HashMap;

const USER_KEY_SET: &str = "user_keys";

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub name: String,
    pub email: String,
    pub can_reset_password: bool,
    pub is_admin: bool,
    pub tasks: UserTaskEntries,
    salt: u64,
    password: String
}

impl Model for User {
    fn fmt_key(identifier: &str) -> String {
        format!("user:{}", identifier)
    }

    fn fetch(connection: &mut Connection, key: &str) -> Option<Self> {
        let result = redis::cmd("GET")
            .arg(Self::fmt_key(key))
            .query::<String>(connection);

        match result {
            Ok(value) => {
                match serde_json::from_str(&value) {
                    Ok(user) => Some(user),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }

    fn store(&self, connection: &mut Connection) -> bool {
        let serialized = match serde_json::to_string(self) {
            Ok(serialized) => serialized,
            Err(_) => return false
        };

        let key = Self::fmt_key(self.username.as_str());
        let exists = redis::cmd("EXISTS")
            .arg(&key)
            .query::<bool>(connection);
        
        match exists {
            Ok(exists) => {
                if exists {
                    return false;
                }
            },
            Err(_) => return false
        };

        let set_result = redis::cmd("SET")
            .arg(&key)
            .arg(serialized)
            .query::<()>(connection);

        let set_key_result = redis::cmd("SADD")
            .arg(USER_KEY_SET)
            .arg(&key)
            .query::<()>(connection);

        set_result.is_ok() && set_key_result.is_ok()
    }

    fn update(&self, connection: &mut Connection) -> bool {
        let serialized = match serde_json::to_string(self) {
            Ok(serialized) => serialized,
            Err(_) => return false
        };

        let key = Self::fmt_key(self.username.as_str());
        let exists = redis::cmd("EXISTS")
            .arg(&key)
            .query::<bool>(connection);

        match exists {
            Ok(exists) => {
                if !exists {
                    return false;
                }
            },
            Err(_) => return false
        };

        let set_result = redis::cmd("SET")
            .arg(&key)
            .arg(serialized)
            .query::<()>(connection);

        set_result.is_ok()
    }
}

impl User {
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
    /// Returns a Result containing the newly created User instance if successful, or an error if the password hashing fails.
    pub fn new(username: String, name: String, email: String, plain_text_password: String, is_admin: bool) -> anyhow::Result<Self> {
        let salt = thread_rng().gen::<u64>();
        let password = bcrypt::hash(format!("{}{}", plain_text_password, salt), bcrypt::DEFAULT_COST)?;

        Ok(Self {
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
}

#[cfg(test)]
mod tests {
    use crate::models::client_user::ClientUser;

    use super::*;

    #[test]
    fn test_new_user() {
        let username = "test-user".to_string();
        let name = "Test User".to_string();
        let email = "test@test.com".to_string();
        let password = "password".to_string();
        let is_admin = false;

        let user = User::new(username.clone(), name.clone(), email.clone(), password.clone(), is_admin).unwrap();

        assert_eq!(user.username, username);
        assert_eq!(user.name, name);
        assert_ne!(user.password, password);
        assert_eq!(user.is_admin, is_admin);
    }

    #[test]
    fn test_validate_password() {
        let username = "test-user".to_string();
        let name = "Test User".to_string();
        let email = "test@test.net".to_string();
        let plain_text_password = "password".to_string();

        let user = User::new(username, name, email, plain_text_password.clone(), false).unwrap();

        assert_eq!(user.validate_password(plain_text_password.as_str()), true);
    }

    #[test]
    fn test_from_user_to_client_user() {
        let username = "test-user".to_string();
        let name = "Test User".to_string();
        let email = "test@test.com".to_string();
        let password = "password".to_string();

        let user = User::new(username.clone(), name.clone(), email.clone(), password.clone(), false).unwrap();
        let client_user: ClientUser = user.into();

        assert_eq!(client_user.username, username);
        assert_eq!(client_user.name, name);
        assert_eq!(client_user.email, email);
        assert_eq!(client_user.is_admin, false);
    }
}