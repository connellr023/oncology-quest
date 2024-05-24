use crate::utilities::parsable::Comment;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};
use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
pub struct UserTask {
    id: i32,
    user_id: i32,
    subtask_id: i32,
    is_completed: bool,
    comment: Comment
}

impl UserTask {
    /// Fetches all user tasks for a user from the database if the user's task cache is outdated.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - A connection pool to the database.
    /// * `user_id` - The ID of the user to fetch the tasks for.
    /// * `task_cache_timestamp` - An optional timestamp for validating the user's task cache.
    /// 
    /// # Returns
    /// 
    /// A list of user tasks if the user's task cache is outdated, otherwise `None`.
    pub async fn fetch_all_if_updated(pool: &Pool<Postgres>, user_id: i32, task_cache_timestamp: Option<DateTime<Utc>>) -> anyhow::Result<Option<Box<[Self]>>> {
        let records = sqlx::query_as!(
            UserTask,
            r#"
            SELECT ut.* 
            FROM user_tasks ut
            JOIN users u ON ut.user_id = u.id
            WHERE ut.user_id = $1 AND u.last_task_update <= $2;
            "#,
            user_id,
            task_cache_timestamp
        )
        .fetch_all(pool)
        .await?;

        match records.is_empty() {
            true => Ok(None),
            false => Ok(Some(records.into_boxed_slice()))
        }
    }

    pub async fn update_is_completed(pool: &Pool<Postgres>, id: i32, is_completed: bool, comment: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_tasks (id, is_completed, comment)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE
            SET is_completed = EXCLUDED.is_completed, comment = EXCLUDED.comment;
            "#,
            id,
            is_completed,
            comment
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}