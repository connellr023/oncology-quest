use sqlx::{Pool, Postgres};

/// Trait representing a Redis model.
pub trait Model: Sized {
    /// Fetches a model from the database without requiring an instance.
    ///
    /// # Arguments
    ///
    /// * `pool` - The Postgres connection pool to use for the fetch operation.
    /// * `primary_key` - The primary key of the model to fetch.
    ///
    /// # Returns
    /// 
    /// A result containing the fetched model or an error if the fetch operation failed or the model does not exist.
    async fn fetch(pool: &Pool<Postgres>, primary_key: i32) -> anyhow::Result<Self>;

    /// Inserts this model into the database.
    ///
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the store operation.
    /// 
    /// # Returns
    /// 
    /// A result indicating whether the store operation was successful.
    async fn insert(&mut self, pool: &Pool<Postgres>) -> anyhow::Result<()>;

    /// Deletes the model from the database without requiring an instance.
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The Postgres connection pool to use for the delete operation.
    /// * `primary_key` - The primary key of the model to delete.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the delete operation was successful.
    async fn delete(pool: &Pool<Postgres>, primary_key: i32) -> anyhow::Result<()>;
}