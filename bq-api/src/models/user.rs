use crate::utilities::parsable::{Email, Name, PlainTextPassword, Username};
use rand::{thread_rng, Rng};
use sqlx::{prelude::FromRow, Pool, Postgres};
use anyhow::anyhow;

pub const USER_KEY_SET: &str = "user_keys";

#[derive(Debug, FromRow)]
pub struct User {
    id: i32,
    username: Username,
    name: Name,
    email: Email,
    can_reset_password: bool,
    is_admin: bool,
    salt: i64,
    password: String,
    login_count: i32
}

impl User {
    pub async fn fetch(pool: &Pool<Postgres>, primary_key: i32) -> anyhow::Result<Self> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1;",
            primary_key
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn insert(&mut self, pool: &Pool<Postgres>) -> anyhow::Result<()> {
        if self.exists(pool).await {
            return Err(anyhow!("User already exists"));
        }

        let row = sqlx::query!(
            r#"
            INSERT INTO users (username, name, email, can_reset_password, is_admin, salt, password, login_count)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
            self.username.as_str(),
            self.name.as_str(),
            self.email.as_str(),
            self.can_reset_password,
            self.is_admin,
            self.salt,
            self.password.as_str(),
            self.login_count
        )
        .fetch_one(pool)
        .await?;
    
        self.id = row.id;
    
        Ok(())
    }

    pub async fn exists(&self, pool: &Pool<Postgres>) -> bool {
        let exists_query = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 OR email = $2)
            "#,
            self.username.as_str(), self.email.as_str()
        )
        .fetch_one(pool)
        .await;

        exists_query.map_or(false, |query| { query.exists.unwrap_or(false) })
    }

    pub async fn delete(pool: &Pool<Postgres>, primary_key: i32) -> anyhow::Result<()> {
        let delete_query = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            primary_key
        )
        .execute(pool)
        .await?;

        if delete_query.rows_affected() == 0 {
            return Err(anyhow!("User does not exist"));
        }
    
        Ok(())
    }

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
    pub fn gen_password_hash(salt: i64, password: &str) -> Option<String> {
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
    /// The ID of the user will be set to -1 indicating that it is not present in the database yet.
    pub fn new(username: Username, name: Name, email: Email, plain_text_password: PlainTextPassword, is_admin: bool) -> Option<Self> {
        let salt = thread_rng().gen::<i64>();
        let password = match Self::gen_password_hash(salt, plain_text_password.as_str()) {
            Some(password) => password,
            None => return None
        };

        Some(Self {
            id: -1,
            username,
            name,
            email,
            password,
            salt,
            is_admin,
            can_reset_password: false,
            login_count: 0
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

    /// Checks if a given user is an admin.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the operation.
    /// * `primary_key` - The primary key of the user to check.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the user is an admin, `false` otherwise.
    pub fn validate_is_admin(pool: Pool<Postgres>, primary_key: u32) -> bool {
        todo!()
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

    pub fn can_reset_password(&self) -> bool {
        self.can_reset_password
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn login_count(&self) -> i32 {
        self.login_count
    }

    pub fn salt(&self) -> i64 {
        self.salt
    }

    pub fn password(&self) -> &str {
        self.password.as_str()
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