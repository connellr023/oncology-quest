use super::prelude::*;
use crate::utilities::parsable::Comment;
use std::collections::HashMap;
use anyhow::anyhow;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserTaskModel {
    id: i32,
    user_id: i32,
    subtask_id: i32,
    rotation_id: i32,
    is_completed: bool,
    comment: Comment
}

#[derive(Serialize)]
pub struct UserTask<S> {
    #[serde(flatten)]
    model: UserTaskModel,

    #[serde(skip)]
    _marker: PhantomData<S>
}

impl<S> UserTask<S> {
    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.model.id
    }

    #[inline(always)]
    pub fn subtask_id(&self) -> i32 {
        self.model.subtask_id
    }

    #[inline(always)]
    pub fn rotation_id(&self) -> i32 {
        self.model.rotation_id
    }
}

impl UserTask<Unsynced> {
    pub fn new(user_id: i32, subtask_id: i32, rotation_id: i32, is_completed: bool, comment: Comment) -> Self {
        Self {
            model: UserTaskModel {
                id: -1,
                user_id,
                subtask_id,
                rotation_id,
                is_completed,
                comment
            },
            _marker: PhantomData
        }
    }

    pub async fn insert(self, pool: &PgPool) -> anyhow::Result<UserTask<Synced>> {
        let mut transaction = pool.begin().await?;
        
        sqlx::query!(
            r#"
            UPDATE users
            SET last_task_update = NOW()
            WHERE id = $1;
            "#,
            self.model.user_id
        )
        .execute(&mut *transaction)
        .await?;

        let row = sqlx::query!(
            r#"
            INSERT INTO user_tasks (user_id, subtask_id, rotation_id, is_completed, comment)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id;
            "#,
            self.model.user_id,
            self.model.subtask_id,
            self.model.rotation_id,
            self.model.is_completed,
            self.model.comment.as_str()
        )
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(UserTask {
            model: UserTaskModel {
                id: row.id,
                user_id: self.model.user_id,
                subtask_id: self.model.subtask_id,
                rotation_id: self.model.rotation_id,
                is_completed: self.model.is_completed,
                comment: self.model.comment
            },
            _marker: PhantomData
        })
    }

    pub async fn exists(&self, pool: &PgPool) -> anyhow::Result<bool> {
        let record = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM user_tasks
                WHERE subtask_id = $1 AND user_id = $2 AND rotation_id = $3
            ) AS "exists!";
            "#,
            self.model.subtask_id,
            self.model.user_id,
            self.model.rotation_id
        )
        .fetch_one(pool)
        .await?;

        Ok(record.exists)
    }
}

impl UserTask<Synced> {
    #[inline(always)]
    fn from(model: UserTaskModel) -> Self {
        Self {
            model,
            _marker: PhantomData
        }
    }

    pub async fn fetch_as_map(pool: &PgPool, user_id: i32, rotation_id: i32) -> anyhow::Result<HashMap<i32, Self>> {        
        let user_tasks = sqlx::query_as!(
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
        .await?;

        let map = user_tasks
            .into_iter()
            .map(|task| { (task.subtask_id, Self::from(task)) })
            .collect::<HashMap<_, _>>();

        Ok(map)
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
}