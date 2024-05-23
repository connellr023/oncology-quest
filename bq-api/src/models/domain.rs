use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};
use serde::Serialize;

#[derive(Serialize, Debug, FromRow)]
pub struct Domain {
    id: i32,
    name: String,
    last_updated: DateTime<Utc>
}

impl Domain {
    /// Checks if a cache is valid by comparing the cache timestamp with the last updated timestamp of the domain.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - A reference to the database connection pool.
    /// * `domain_id` - The ID of the domain.
    /// * `cache_timestamp` - The timestamp of the cache to be checked.
    /// 
    /// # Returns
    /// 
    /// A boolean wrapped in a Result indicating whether the cache is valid. There will be an error if a database error occurs.
    pub async fn is_cache_valid(pool: &Pool<Postgres>, domain_id: i32, cache_timestamp: Option<DateTime<Utc>>) -> anyhow::Result<bool> {        
        let cache_timestamp = match cache_timestamp {
            Some(cache_timestamp) => cache_timestamp,
            None => return Ok(false)
        };
        
        let last_updated = sqlx::query!(
            r#"
            SELECT last_updated FROM domains WHERE id = $1;
            "#,
            domain_id
        )
        .fetch_one(pool)
        .await?
        .last_updated;

        // If the cache timestamp is greater than or equal to the last updated timestamp, then the cache is valid.
        Ok(cache_timestamp >= last_updated)
    }

    pub async fn fetch_all(pool: &Pool<Postgres>) -> anyhow::Result<Box<[Self]>> {
        let domains = sqlx::query_as!(
            Domain,
            r#"
            SELECT * FROM domains;
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(domains.into_boxed_slice())
    }
}