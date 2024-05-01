use redis::Connection;
use serde::{Serialize, Deserialize};

/// Trait representing a Redis model.
pub trait RedisModel {
    /// Fetches the value associated with the given key from Redis.
    ///
    /// # Arguments
    ///
    /// * `connection` - The Redis connection to use for the fetch operation.
    /// * `key` - The key to fetch the value for.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` containing the fetched model, or `None` if the key does not exist.
    fn fetch(&self, connection: &mut Connection, key: &str) -> Option<Self>
    where Self: Serialize + for<'de> Deserialize<'de> + Sized;

    /// Stores the model in Redis.
    ///
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the store operation.
    /// 
    /// # Returns
    ///
    /// A boolean indicating whether the store operation was successful.
    fn store(&self, connection: &mut Connection) -> bool;
}