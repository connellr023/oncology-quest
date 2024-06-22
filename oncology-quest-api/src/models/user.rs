use super::prelude::*;
use crate::utilities::parsable::{Name, PlainTextPassword, ResetToken, Username};
use actix_session::Session;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use anyhow::anyhow;

const PASSWORD_RESET_TOKEN_LENGTH: usize = 4;
const SESSION_USER_ID_KEY: &str = "uid";

#[derive(Debug, Clone)]
struct UserModel {
    id: i32,
    username: Username,
    name: Name,
    is_admin: bool,
    salt: i64,
    password: String,
    login_count: i32
}

/// Wrapper around a UserModel struct that indicates if the user is synced with the database.
pub struct User<S>(UserModel, PhantomData<S>);

impl<S> User<S> {
    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.0.id
    }

    #[inline(always)]
    pub fn username(&self) -> &Username {
        &self.0.username
    }

    #[inline(always)]
    pub fn name(&self) -> &Name {
        &self.0.name
    }

    #[inline(always)]
    pub fn is_admin(&self) -> bool {
        self.0.is_admin
    }

    #[inline(always)]
    pub fn login_count(&self) -> i32 {
        self.0.login_count
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
    pub fn is_valid_password(&self, plain_text_password: &str) -> bool {
        bcrypt::verify(format!("{}{}", plain_text_password, self.0.salt), &self.0.password).unwrap_or(false)
    }

    /// Deletes a user from the database by their ID.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the operation.
    /// * `user_id` - The ID of the user to delete.
    /// * `include_admins` - A flag indicating whether to allow deletion of admin users.
    /// 
    /// # Returns
    /// 
    /// Returns a result containing `true` if the user was deleted, `false` otherwise and an error if the operation failed.
    async fn delete(pool: &PgPool, user_id: i32, include_admins: bool) -> anyhow::Result<bool> {
        let mut transaction = pool.begin().await?;

        let rows_affected_tasks = sqlx::query!(
            "DELETE FROM user_tasks WHERE user_id = $1;",
            user_id
        )
        .execute(&mut *transaction)
        .await?
        .rows_affected();

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

        let rows_affected_users = query
            .execute(&mut *transaction)
            .await?
            .rows_affected();

        transaction.commit().await?;
    
        Ok(rows_affected_users > 0 || rows_affected_tasks > 0)
    }
}

impl User<Unsynced> {
    /// Creates a new User instance with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `name` - The name of the user.
    /// * `is_admin` - A flag indicating whether the user is an admin.
    /// * `plain_text_password` - The plain text password of the user.
    ///
    /// # Returns
    ///
    /// Returns a new User instance if the password was successfully hashed, `None` otherwise.
    /// The ID of the user will be set to -1 indicating that it is not present in the database yet.
    pub fn new(username: Username, name: Name, is_admin: bool, plain_text_password: PlainTextPassword) -> anyhow::Result<Self> {
        let salt = thread_rng().gen::<i64>();
        let password = Self::gen_password_hash(salt, plain_text_password.as_str())?;

        Ok(User(
            UserModel {
                id: -1,
                username,
                name,
                is_admin,
                salt,
                password,
                login_count: 0
            },
            PhantomData
        ))
    }

    /// Checks if this user exists in the database.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the operation.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the user exists, `false` otherwise.
    pub async fn exists(&self, pool: &PgPool) -> anyhow::Result<bool> {
        let record = sqlx::query!(
            r#"
            SELECT
            EXISTS(SELECT 1 FROM users WHERE username = $1)
            AS "exists!";
            "#,
            self.0.username.as_str()
        )
        .fetch_one(pool)
        .await?;

        Ok(record.exists)
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
    pub async fn insert(self, pool: &PgPool) -> anyhow::Result<User<Synced>> {
        let row = sqlx::query!(
            r#"
            INSERT INTO users (username, name, is_admin, salt, password)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            self.0.username.as_str(),
            self.0.name.as_str(),
            self.0.is_admin,
            self.0.salt,
            self.0.password.as_str()
        )
        .fetch_one(pool)
        .await?;

        Ok(User(
            UserModel {
                id: row.id,
                username: self.0.username,
                name: self.0.name,
                is_admin: self.0.is_admin,
                salt: self.0.salt,
                password: self.0.password,
                login_count: self.0.login_count
            },
            PhantomData
        ))
    }

    /// Updates the user's password if the reset token is valid and has not expired.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the operation.
    /// * `username` - The username of the user to update the password for.
    /// * `plain_text_password` - The new plain text password to set.
    /// * `reset_token` - The reset token to validate.
    /// 
    /// # Returns
    /// 
    /// Returns a result containing `true` if the password was updated, `false` otherwise and an error if the operation failed.
    pub async fn update_password(pool: &PgPool, username: &str, plain_text_password: &str, reset_token: &str) -> anyhow::Result<bool> {
        let record = match sqlx::query!(
            r#"
            SELECT password, salt
            FROM users
            WHERE username = $1 AND password_reset_timestamp > NOW() AND password_reset_token = $2;
            "#,
            username,
            reset_token
        )
        .fetch_optional(pool)
        .await? {
            Some(record) => record,
            None => return Ok(false)
        };

        let new_hashed_password = Self::gen_password_hash(record.salt, plain_text_password)?;
        let rows_affected = sqlx::query!(
            r#"
            UPDATE users
            SET password = $1, password_reset_timestamp = NOW(), password_reset_token = NULL
            WHERE username = $2;
            "#,
            new_hashed_password,
            username
        )
        .execute(pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Ok(false)
        }

        Ok(true)
    }
}

impl User<Synced> {
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
    /// Returns either a Regular or Admin user if the login was successful, an error otherwise.
    pub async fn login(pool: &PgPool, username: &str, plain_text_password: &str) -> anyhow::Result<Self> {
        let mut transaction = pool.begin().await?;
        
        let result = Self(
            sqlx::query_as!(
                UserModel,
                r#"
                UPDATE users
                SET login_count = login_count + 1
                WHERE username = $1
                RETURNING id, username, name, is_admin, salt, password, login_count;
                "#,
                username
            )
            .fetch_one(&mut *transaction)
            .await?,
            PhantomData
        );

        if !result.is_valid_password(plain_text_password) {
            transaction.rollback().await?;
            return Err(anyhow!("Invalid password"));
        }

        transaction.commit().await?;

        Ok(result)
    }

    /// Fetches a user from the database by their ID.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the operation.
    /// * `user_id` - The ID of the user to fetch.
    /// 
    /// # Returns
    /// 
    /// Returns a Regular user if the user is not an admin, an Admin user if the user is an admin, an error otherwise.
    pub async fn fetch_by_id(pool: &PgPool, user_id: i32) -> anyhow::Result<Self> {
        let result = sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, username, name, is_admin, salt, password, login_count
            FROM users
            WHERE id = $1;
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Self(result, PhantomData))
    }

    pub async fn validate_admin_session(pool: &PgPool, session: &Session) -> anyhow::Result<bool> {
        let user_id = UserSession::extract_user_id(session)
            .ok_or_else(|| anyhow!("User ID not found in session"))?;

        let exists = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE id = $1 AND is_admin = TRUE) AS "exists!";
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?
        .exists;

        Ok(exists)
    }

    /// Generates a random password reset token.
    #[inline(always)]
    fn generate_reset_token() -> ResetToken {
        let token = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(PASSWORD_RESET_TOKEN_LENGTH)
            .map(char::from)
            .collect();

        ResetToken::parse(token).unwrap()
    }

    /// Allows a user to reset their password by generating a reset token and setting the expiration time.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the operation.
    /// * `user_id` - The ID of the user to allow password reset for.
    /// * `expiration_hours` - The number of hours the reset token will be valid for.
    /// 
    /// # Returns
    /// 
    /// Returns a tuple containing the expiration time and the reset token if successful, an error otherwise.
    pub async fn allow_reset_password(pool: &PgPool, user_id: i32, expiration_hours: i32) -> anyhow::Result<(DateTime<Utc>, ResetToken)> {
        let token = Self::generate_reset_token();
        let row = sqlx::query!(
            r#"
            UPDATE users
            SET password_reset_timestamp = NOW() + make_interval(hours => $1), password_reset_token = $2
            WHERE id = $3
            RETURNING password_reset_timestamp;
            "#,
            expiration_hours,
            token.as_str(),
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok((row.password_reset_timestamp, token))
    }

    #[inline(always)]
    pub async fn delete_self(self, pool: &PgPool) -> anyhow::Result<bool> {
        Self::delete(pool, self.id(), true).await
    }

    #[inline(always)]
    pub async fn delete_other(pool: &PgPool, user_id: i32) -> anyhow::Result<bool> {
        Self::delete(pool, user_id, false).await
    }
}

pub struct UserSession;

impl UserSession {
    /// Checks if the provided session is valid.
    #[inline(always)]
    pub fn extract_user_id(session: &Session) -> Option<i32> {
        session.get::<i32>(SESSION_USER_ID_KEY)
            .ok()?
            .clone()
    }

    #[inline(always)]
    pub fn is_insert_ok(session: &Session, user_id: i32) -> bool {
        session.insert(SESSION_USER_ID_KEY, user_id).is_ok()
    }

    #[inline(always)]
    pub fn clear(session: &Session) {
        session.remove(SESSION_USER_ID_KEY);
    }
}