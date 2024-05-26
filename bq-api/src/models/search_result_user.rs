use sqlx::{Pool, Postgres};
use serde::Serialize;

use super::{client_user::ClientUser, user_task::UserTask};

#[derive(Serialize)]
pub struct SearchResultUser {
    user: ClientUser,
    tasks: Box<[UserTask]>
}

impl SearchResultUser {
    pub async fn text_search(pool: &Pool<Postgres>, query: &str, limit: i64) -> anyhow::Result<Box<[Self]>> {
        let users = sqlx::query_as!(
            ClientUser,
            r#"
            SELECT users.id, users.username, users.name, users.email, users.is_admin, users.login_count
            FROM users
            WHERE (username ILIKE $1 OR name ILIKE $1 OR email ILIKE $1)
            AND is_admin = FALSE
            LIMIT $2;
            "#,
            format!("%{}%", query),
            limit
        )
        .fetch_all(pool)
        .await?;

        let mut results = Vec::with_capacity(users.len());

        for user in users {
            let tasks = UserTask::fetch_all(pool, user.id).await?;

            results.push(Self {
                user,
                tasks
            });
        }

        Ok(results.into_boxed_slice())
    }
}
