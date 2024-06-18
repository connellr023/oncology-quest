use super::prelude::*;
use crate::utilities::parsable::Comment;
use std::collections::HashMap;
use anyhow::anyhow;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UserTaskModel {
    id: i32,
    user_id: i32,
    subtask_id: i32,
    rotation_id: i32,
    is_completed: bool,
    comment: Comment
}

pub struct UserTask<SyncState>(UserTaskModel, PhantomData<SyncState>);

impl UserTask<DatabaseUnsynced> {
    pub fn new(user_id: i32, subtask_id: i32, rotation_id: i32, is_completed: bool, comment: Comment) -> Self {
        Self(
            UserTaskModel {
                id: -1,
                user_id,
                subtask_id,
                rotation_id,
                is_completed,
                comment
            },
            PhantomData
        )
    }

    pub async fn insert(self, pool: &PgPool) -> anyhow::Result<UserTask<DatabaseSynced>> {
        let mut transaction = pool.begin().await?;
        
        sqlx::query!(
            r#"
            UPDATE users
            SET last_task_update = NOW()
            WHERE id = $1;
            "#,
            self.0.user_id
        )
        .execute(&mut *transaction)
        .await?;

        let row = sqlx::query!(
            r#"
            INSERT INTO user_tasks (user_id, subtask_id, rotation_id, is_completed, comment)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id;
            "#,
            self.0.user_id,
            self.0.subtask_id,
            self.0.rotation_id,
            self.0.is_completed,
            self.0.comment.as_str()
        )
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(UserTask(
            UserTaskModel {
                id: row.id,
                user_id: self.0.user_id,
                subtask_id: self.0.subtask_id,
                rotation_id: self.0.rotation_id,
                is_completed: self.0.is_completed,
                comment: self.0.comment
            },
            PhantomData
        ))
    }
}

impl UserTask<DatabaseSynced> {
    pub async fn exists(&self, pool: &PgPool) -> anyhow::Result<bool> {
        let record = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM user_tasks
                WHERE subtask_id = $1 AND user_id = $2 AND rotation_id = $3
            ) AS "exists!";
            "#,
            self.0.subtask_id,
            self.0.user_id,
            self.0.rotation_id
        )
        .fetch_one(pool)
        .await?;

        Ok(record.exists)
    }

    pub async fn fetch_as_map(pool: &PgPool, user_id: i32, rotation_id: i32, task_cache_timestamp: Option<DateTime<Utc>>) -> anyhow::Result<Option<HashMap<i32, Self>>> {        
        let user_tasks = match task_cache_timestamp {
            Some(timestamp) => {
                sqlx::query_as!(
                    UserTaskModel,
                    r#"
                    SELECT ut.* 
                    FROM user_tasks ut
                    JOIN users u ON ut.user_id = u.id
                    WHERE ut.user_id = $1 AND u.last_task_update <= $2 AND ut.rotation_id = $3;
                    "#,
                    user_id,
                    timestamp,
                    rotation_id
                )
                .fetch_all(pool)
                .await?
            },
            None => {
                sqlx::query_as!(
                    UserTaskModel,
                    r#"
                    SELECT * 
                    FROM user_tasks
                    WHERE user_id = $1 AND rotation_id = $2;
                    "#,
                    user_id,
                    rotation_id
                )
                .fetch_all(pool)
                .await?
            }
        };

        if user_tasks.is_empty() {
            return Ok(None);
        }

        let map = user_tasks
            .into_iter()
            .map(|task| { (task.subtask_id, Self(task, PhantomData)) })
            .collect::<HashMap<_, _>>();

        Ok(Some(map))
    }

    pub async fn update(pool: &PgPool, id: i32, user_id: i32, is_completed: bool, comment: &str) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;
        
        sqlx::query!(
            r#"
            UPDATE users
            SET last_task_update = NOW()
            WHERE id = $1;
            "#,
            user_id
        )
        .execute(&mut *transaction)
        .await?;
        
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
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        if update_query.rows_affected() == 0 {
            return Err(anyhow!("User task does not exist."));
        }

        Ok(())
    }

    pub fn id(&self) -> i32 {
        self.0.id
    }

    pub fn subtask_id(&self) -> i32 {
        self.0.subtask_id
    }

    pub fn rotation_id(&self) -> i32 {
        self.0.rotation_id
    }
}