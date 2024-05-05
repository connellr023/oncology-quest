use redis::Connection;
use serde::{Serialize, Deserialize};

/// Trait representing a Redis model.
pub trait Model {
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
    fn fetch(connection: &mut Connection, key: &str) -> Option<Self>
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

    /// Updates the model in Redis.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the update operation.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the update operation was successful.
    fn update(&self, connection: &mut Connection) -> bool;

    /// Checks if a model exists in Redis.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the exists operation.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the model exists in Redis.
    fn exists(&self, connection: &mut Connection) -> bool {
        let exists = redis::cmd("EXISTS")
            .arg(self.key())
            .query::<bool>(connection);

        match exists {
            Ok(exists) => exists,
            Err(_) => false
        }
    }

    /// The key format for this model.
    fn fmt_key(identifier: &str) -> String;

    /// Returns the key for the model.
    fn key(&self) -> String;
}