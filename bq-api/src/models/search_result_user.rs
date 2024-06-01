use super::{client_user::ClientUser, user_task::UserTask};
use sqlx::{Pool, Postgres};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct SearchResultUser {
    user: ClientUser,
    tasks: HashMap<i32, UserTask>
}

impl SearchResultUser {
    pub async fn text_search_as_map(pool: &Pool<Postgres>, query: &str, limit: i64) -> anyhow::Result<HashMap<i32, Self>> {
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
            let tasks = UserTask::fetch_all_as_map(pool, user.id).await?;

            results.push(Self {
                user,
                tasks
            });
        }

        let map = results
            .into_iter()
            .map(|result| { (result.user.id, result) })
            .collect::<HashMap<_, _>>();

        Ok(map)
    }
}
