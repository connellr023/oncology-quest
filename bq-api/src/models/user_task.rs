use crate::utilities::parsable::Comment;
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
    pub async fn fetch_all(pool: &Pool<Postgres>, user_id: i32) -> anyhow::Result<Box<[Self]>> {
        let records = sqlx::query_as!(
            UserTask,
            r#"
            SELECT * FROM user_tasks WHERE user_id = $1;
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(records.into_boxed_slice())
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