use crate::utilities::parsable::Comment;
use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug, FromRow)]
pub struct UserTask {
    id: i32,
    user_id: i32,
    subtask_id: i32,
    is_completed: bool,
    comment: Comment
}

impl UserTask {
    pub async fn fetch_by_user_id(pool: &Pool<Postgres>, user_id: i32) -> anyhow::Result<Vec<Self>> {
        let records = sqlx::query_as!(
            UserTask,
            r#"
            SELECT * FROM user_tasks WHERE user_id = $1;
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(records)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    pub fn comment(&self) -> &Comment {
        &self.comment
    }

    pub fn subtask_id(&self) -> i32 {
        self.subtask_id
    }
}