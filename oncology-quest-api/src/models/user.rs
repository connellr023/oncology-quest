use crate::utilities::parsable::{Email, Name, PlainTextPassword, Username};
use chrono::{DateTime, Utc};
use rand::{thread_rng, Rng};
use sqlx::{FromRow, PgPool};
use anyhow::anyhow;

#[derive(Debug, FromRow, Clone)]
pub struct User {
    id: i32,
    username: Username,
    name: Name,
    email: Email,
    is_admin: bool,
    salt: i64,
    password: String,
    login_count: i32,
    
    // These fields are used in the database but not in the code.
    // password_reset_timestamp: DateTime<Utc>,
    // last_task_update: DateTime<Utc>
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
    /// Returns a new User instance if the password was successfully hashed, `None` otherwise.
    /// The ID of the user will be set to -1 indicating that it is not present in the database yet.
    pub fn new(username: Username, name: Name, email: Email, plain_text_password: PlainTextPassword, is_admin: bool) -> anyhow::Result<Self> {
        let salt = thread_rng().gen::<i64>();
        let password = Self::gen_password_hash(salt, plain_text_password.as_str())?;

        Ok(Self {
            id: -1,
            username,
            name,
            email,
            password,
            salt,
            is_admin,
            login_count: 0
        })
    }

    pub async fn fetch_by_id(pool: &PgPool, user_id: i32) -> anyhow::Result<Self> {
        let result = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, name, email, is_admin, salt, password, login_count FROM users WHERE id = $1;
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn allow_reset_password(pool: &PgPool, user_id: i32, expiration_hours: i32) -> anyhow::Result<DateTime<Utc>> {
        let row = sqlx::query!(
            r#"
            UPDATE users
            SET password_reset_timestamp = NOW() + make_interval(hours => $1)
            WHERE id = $2
            RETURNING password_reset_timestamp;
            "#,
            expiration_hours,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(row.password_reset_timestamp)
    }

    pub async fn update_password(pool: &PgPool, username: &str, plain_text_password: &str) -> anyhow::Result<()> {
        let record = match sqlx::query!(
            r#"
            SELECT password, salt
            FROM users
            WHERE username = $1 AND password_reset_timestamp > NOW();
            "#,
            username
        )
        .fetch_optional(pool)
        .await? {
            Some(record) => record,
            None => return Err(anyhow!("User does not exist or cannot reset password"))
        };

        let new_hashed_password = Self::gen_password_hash(record.salt, plain_text_password)?;
        let rows_affected = sqlx::query!(
            r#"
            UPDATE users
            SET password = $1, password_reset_timestamp = NOW()
            WHERE username = $2;
            "#,
            new_hashed_password, username
        )
        .execute(pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(anyhow::anyhow!("No user found with the given username, or the password was not updated"));
        }

        Ok(())
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
    pub async fn validate_is_admin(pool: &PgPool, primary_key: i32) -> bool {
        match sqlx::query!(
            r#"
            SELECT is_admin FROM users WHERE id = $1;
            "#,
            primary_key
        )
        .fetch_one(pool)
        .await {
            Ok(query) => query.is_admin,
            Err(_) => false
        }
    }

    /// Validates a user's login credentials and increments the login count.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the operation.
    /// * `username` - The username of the user to validate.
    /// * `plain_text_password` - The plain text password to validate.
    /// 
    /// # Returns
    /// 
    /// Returns the user if the login was successful, an error otherwise.
    pub async fn validate_login(pool: &PgPool, username: &str, plain_text_password: &str) -> anyhow::Result<Self> {
        let mut transaction = pool.begin().await?;
        
        let result_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET login_count = login_count + 1
            WHERE username = $1
            RETURNING id, username, name, email, is_admin, salt, password, login_count;
            "#,
            username
        )
        .fetch_one(&mut *transaction)
        .await?;

        if !result_user.validate_password(plain_text_password) {
            transaction.rollback().await?;
            return Err(anyhow!("Invalid password"));
        }

        transaction.commit().await?;

        Ok(result_user)
    }

    /// Inserts the user into the database and sets the user's ID.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the operation.
    /// 
    /// # Returns
    /// 
    /// Returns an error if the insert operation fails.
    pub async fn insert(&mut self, pool: &PgPool) -> anyhow::Result<()> {
        let row = sqlx::query!(
            r#"
            INSERT INTO users (username, name, email, is_admin, salt, password)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            self.username.as_str(),
            self.name.as_str(),
            self.email.as_str(),
            self.is_admin,
            self.salt,
            self.password.as_str()
        )
        .fetch_one(pool)
        .await?;
    
        self.id = row.id;
    
        Ok(())
    }

    pub async fn exists(&self, pool: &PgPool) -> anyhow::Result<bool> {
        let record = sqlx::query!(
            r#"
            SELECT
            EXISTS(SELECT 1 FROM users WHERE username = $1 OR email = $2)
            AS "exists!";
            "#,
            self.username.as_str(), self.email.as_str()
        )
        .fetch_one(pool)
        .await?;

        Ok(record.exists)
    }

    pub async fn delete(pool: &PgPool, user_id: i32, include_admins: bool) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        sqlx::query!(
            "DELETE FROM user_tasks WHERE user_id = $1;",
            user_id
        )
        .execute(&mut *transaction)
        .await?;

        let query = match include_admins {
            true => sqlx::query!(
                "DELETE FROM users WHERE id = $1;",
                user_id
            ),
            false => sqlx::query!(
                "DELETE FROM users WHERE id = $1 AND is_admin = FALSE;",
                user_id
            )
        };

        query
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;
    
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
    /// Returns the hashed password if successful, an error otherwise.
    pub fn gen_password_hash(salt: i64, password: &str) -> anyhow::Result<String> {
        Ok(bcrypt::hash(format!("{}{}", password, salt), bcrypt::DEFAULT_COST)?)
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