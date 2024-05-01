use super::redis_model::RedisModel;
use redis::Connection;
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};

#[derive(Serialize, Deserialize)]
pub struct User {
    username: String,
    name: String,
    password: String,
    salt: u64,
    can_reset_password: bool,
    is_admin: bool,
    tasks: Vec<Vec<usize>>
}

impl RedisModel for User {
    fn fetch(&self, connection: &mut Connection, key: &str) -> Option<Self> {
        let result = redis::cmd("GET")
            .arg(key)
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
        if let Ok(serialized) = serde_json::to_string(self) {
            let result: redis::RedisResult<()> = redis::cmd("SET")
                .arg(self.username.clone())
                .arg(serialized)
                .query(connection);
    
            return result.is_ok();
        }
        
        false
    }
}

impl User {
    // fn validate_params(username: &str, name: &str, plain_text_password: &str) -> bool {
    //     if plain_text_password.is_empty() {
    //         return false;
    //     }
    
    //     let username_pattern = Regex::new(r"^[a-zA-Z0-9\-\_\.]{0,25}$").unwrap();
    //     let name_pattern = Regex::new(r"^[a-zA-Z]+(\s[a-zA-Z]+)*$").unwrap();

    //     username_pattern.is_match(username) && name_pattern.is_match(name)
    // }

    /// Creates a new User instance with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `name` - The name of the user.
    /// * `plain_text_password` - The plain text password of the user.
    /// * `is_admin` - A flag indicating whether the user is an admin or not.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the newly created User instance if successful, or an error if the password hashing fails.
    pub fn new(username: String, name: String, plain_text_password: String, is_admin: bool) -> anyhow::Result<Self> {
        let salt = thread_rng().gen::<u64>();
        let password = bcrypt::hash(format!("{}{}", plain_text_password, salt), bcrypt::DEFAULT_COST)?;

        Ok(Self {
            username,
            name,
            password,
            salt,
            is_admin,
            can_reset_password: false,
            tasks: vec![]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        let username = "test-user".to_string();
        let name = "Test User".to_string();
        let password = "password".to_string();
        let is_admin = false;

        let user = User::new(username.clone(), name.clone(), password.clone(), is_admin).unwrap();

        assert_eq!(user.username, username);
        assert_eq!(user.name, name);
        assert_ne!(user.password, password);
        assert_eq!(user.is_admin, is_admin);
    }
}