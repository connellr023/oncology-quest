use crate::utilities::parsable::Comment;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};
use serde::Serialize;
use anyhow::anyhow;

#[derive(Debug, FromRow, Serialize)]
pub struct UserTask {
    id: i32,
    user_id: i32,
    subtask_id: i32,
    is_completed: bool,
    comment: Comment
}

impl UserTask {
    pub fn new(user_id: i32, subtask_id: i32, is_completed: bool, comment: Comment) -> Self {
        Self {
            id: -1,
            user_id,
            subtask_id,
            is_completed,
            comment
        }
    }

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

    pub async fn insert(&mut self, pool: &Pool<Postgres>) -> anyhow::Result<()> {
        let row = sqlx::query!(
            r#"
            INSERT INTO user_tasks (user_id, subtask_id, is_completed, comment)
            VALUES ($1, $2, $3, $4)
            RETURNING id;
            "#,
            self.user_id,
            self.subtask_id,
            self.is_completed,
            self.comment.as_str()
        )
        .fetch_one(pool)
        .await?;

        self.id = row.id;

        Ok(())
    }

    pub async fn update(pool: &Pool<Postgres>, id: i32, user_id: i32, is_completed: bool, comment: &str) -> anyhow::Result<()> {
        let update_query = sqlx::query!(
            r#"
            UPDATE user_tasks
            SET is_completed = $1, comment = $2
            WHERE id = $3 AND user_id = $4;
            "#,
            is_completed,
            comment,
            id,
            user_id
        )
        .execute(pool)
        .await?;

        if update_query.rows_affected() == 0 {
            return Err(anyhow!("User task does not exist."));
        }

        Ok(())
    }
}