use redis::Connection;
use serde::{Serialize, Deserialize};

/// Trait representing a Redis model.
pub trait Model: Serialize + for<'de> Deserialize<'de> + Sized {
    /// Fetches the value associated with the given key from Redis.
    ///
    /// # Arguments
    ///
    /// * `connection` - The Redis connection to use for the fetch operation.
    /// * `identifier` - The identifier of the model to fetch.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` containing the fetched model, or `None` if the key does not exist.
    fn fetch(connection: &mut Connection, identifier: &str) -> Option<Self> {
        let result = redis::cmd("GET")
            .arg(Self::fmt_key(identifier))
            .query::<String>(connection);

        match result {
            Ok(value) => {
                match serde_json::from_str::<Self>(&value) {
                    Ok(value) => Some(value),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }

    /// Stores the model in Redis.
    ///
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the store operation.
    /// 
    /// # Returns
    ///
    /// A boolean indicating whether the store operation was successful.
    fn store(&self, connection: &mut Connection) -> bool {
        let serialized = match serde_json::to_string(self) {
            Ok(serialized) => serialized,
            Err(_) => return false
        };

        if self.exists(connection) {
            return false;
        }

        let key = self.key();

        let set_result = redis::cmd("SET")
            .arg(&key)
            .arg(serialized)
            .query::<()>(connection);

        set_result.is_ok()
    }

    /// Updates the model in Redis.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the update operation.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the update operation was successful.
    fn update(&self, connection: &mut Connection) -> bool {
        let serialized = match serde_json::to_string(self) {
            Ok(serialized) => serialized,
            Err(_) => return false
        };

        if !self.exists(connection) {
            return false;
        }

        let set_result = redis::cmd("SET")
            .arg(self.key())
            .arg(serialized)
            .query::<()>(connection);

        set_result.is_ok()
    }

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

    /// Formats the key for this model.
    fn fmt_key(identifier: &str) -> String;

    /// Returns the key for the model.
    fn key(&self) -> String;
}