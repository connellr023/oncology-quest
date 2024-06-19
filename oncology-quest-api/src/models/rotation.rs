use super::prelude::*;
use crate::query_many;
use crate::utilities::parsable::Name;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RotationModel {
    id: i32,
    name: Name,
    last_updated: DateTime<Utc>
}

#[derive(Serialize)]
pub struct Rotation<S>(RotationModel, PhantomData<S>);

impl Rotation<Unknown> {
    pub fn new(name: Name) -> Self {
        Self(
            RotationModel {
                id: 0,
                name,
                last_updated: Utc::now()
            },
            PhantomData
        )
    }
    
    pub async fn insert(self, pool: &PgPool) -> anyhow::Result<Rotation<InDatabase>> {
        let row = sqlx::query!(
            r#"
            INSERT INTO rotations (name)
            VALUES ($1)
            RETURNING id, last_updated;
            "#,
            self.0.name.as_str()
        )
        .fetch_one(pool)
        .await?;

        Ok(Rotation(
            RotationModel {
                id: row.id,
                name: self.0.name,
                last_updated: row.last_updated
            },
            PhantomData
        ))
    }
}

impl Rotation<InDatabase> {
    pub async fn delete(pool: &PgPool, rotation_id: i32) -> anyhow::Result<()> {
        let mut transaction = pool.begin().await?;

        query_many!(&mut *transaction, rotation_id,
            "DELETE FROM user_tasks WHERE subtask_id = ANY(SELECT id FROM subtasks WHERE rotation_id = $1)",
            "DELETE FROM subtasks WHERE rotation_id = $1",
            "DELETE FROM tasks WHERE rotation_id = $1",
            "DELETE FROM supertasks WHERE rotation_id = $1",
            "DELETE FROM rotations WHERE id = $1",
        );

        transaction.commit().await?;

        Ok(())
    }

    /// Checks if a cache is valid by comparing the cache timestamp with the last updated timestamp of the rotation.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - A reference to the database connection pool.
    /// * `rotation_id` - The ID of the rotation.
    /// * `cache_timestamp` - The timestamp of the cache to be checked.
    /// 
    /// # Returns
    /// 
    /// A boolean wrapped in a Result indicating whether the cache is valid. There will be an error if a database error occurs.
    pub async fn is_cache_valid(pool: &PgPool, rotation_id: i32, cache_timestamp: Option<DateTime<Utc>>) -> anyhow::Result<bool> {        
        let cache_timestamp = match cache_timestamp {
            Some(cache_timestamp) => cache_timestamp,
            None => return Ok(false)
        };
        
        let last_updated = sqlx::query!(
            r#"
            SELECT last_updated FROM rotations WHERE id = $1;
            "#,
            rotation_id
        )
        .fetch_one(pool)
        .await?
        .last_updated;

        // If the cache timestamp is greater than or equal to the last updated timestamp, then the cache is valid.
        Ok(cache_timestamp >= last_updated)
    }

    pub async fn fetch_all_as_map(pool: &PgPool) -> anyhow::Result<HashMap<i32, Self>> {
        let rotations = sqlx::query_as!(
            RotationModel,
            r#"
            SELECT * FROM rotations;
            "#
        )
        .fetch_all(pool)
        .await?;

        let map = rotations
            .into_iter()
            .map(|rotation| { (rotation.id, Self(rotation, PhantomData)) })
            .collect::<HashMap<_, _>>();

        Ok(map)
    }

    pub async fn exists(pool: &PgPool, rotation_id: i32) -> anyhow::Result<bool> {
        let exists_query = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT id FROM rotations WHERE id = $1);
            "#,
            rotation_id
        )
        .fetch_one(pool)
        .await?
        .exists;

        Ok(exists_query.unwrap_or(false))
    }

    pub fn id(&self) -> i32 {
        self.0.id
    }

    pub fn last_updated(&self) -> DateTime<Utc> {
        self.0.last_updated
    }
}