use super::prelude::*;
use crate::utilities::parsable::Name;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct RotationModel {
    id: i32,
    name: Name,
    last_updated: DateTime<Utc>
}

#[derive(Serialize, Clone)]
pub struct Rotation<S> {
    #[serde(flatten)]
    model: RotationModel,

    #[serde(skip)]
    _marker: PhantomData<S>
}

impl<S> Rotation<S> {
    #[inline(always)]
    pub fn id(&self) -> i32 {
        self.model.id
    }

    #[inline(always)]
    pub fn last_updated(&self) -> DateTime<Utc> {
        self.model.last_updated
    }
}

impl Rotation<Unsynced> {
    pub fn new(name: Name) -> Self {
        Self {
            model: RotationModel {
                id: 0,
                name,
                last_updated: Utc::now()
            },
            _marker: PhantomData
        }
    }

    pub async fn insert(self, pool: &PgPool) -> Result<Rotation<Synced>> {
        let row = sqlx::query!(
            r#"
            INSERT INTO rotations (name)
            VALUES ($1)
            RETURNING id, last_updated;
            "#,
            self.model.name.as_str()
        )
        .fetch_one(pool)
        .await?;

        Ok(Rotation {
            model: RotationModel {
                id: row.id,
                name: self.model.name,
                last_updated: row.last_updated
            },
            _marker: PhantomData
        })
    }
}

impl Rotation<Synced> {
    #[inline(always)]
    fn from(model: RotationModel) -> Self {
        Self {
            model,
            _marker: PhantomData
        }
    }

    pub async fn delete(pool: &PgPool, rotation_id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM rotations WHERE id = $1;
            "#,
            rotation_id
        )
        .execute(pool)
        .await?;

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
    pub async fn is_cache_valid(pool: &PgPool, rotation_id: i32, cache_timestamp: Option<DateTime<Utc>>) -> Result<bool> {
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

    pub async fn fetch_all_as_map(pool: &PgPool) -> Result<HashMap<i32, Self>> {
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
            .map(|rotation| { (rotation.id, Self::from(rotation)) })
            .collect::<HashMap<_, _>>();

        Ok(map)
    }

    pub async fn exists(pool: &PgPool, rotation_id: i32) -> Result<bool> {
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
}
