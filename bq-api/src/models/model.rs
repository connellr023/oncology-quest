use redis::Connection;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Ok};

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
    /// A result containing the fetched model or an error if the fetch operation failed or the model does not exist.
    fn fetch(connection: &mut Connection, identifier: &str) -> anyhow::Result<Self> {
        let result = redis::cmd("GET")
            .arg(Self::fmt_key(identifier))
            .query::<String>(connection)?;

        Ok(serde_json::from_str::<Self>(&result)?)
    }

    /// Stores the model in Redis.
    ///
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the store operation.
    /// 
    /// # Returns
    /// 
    /// A result indicating whether the store operation was successful.
    fn store(&self, connection: &mut Connection) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(self)?;

        if self.exists(connection) {
            return Err(anyhow!("Model already exists"));
        }

        let key = self.key();

        redis::cmd("SET")
            .arg(&key)
            .arg(serialized)
            .query::<()>(connection)?;

        Ok(())
    }

    /// Updates the model in Redis.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the update operation.
    /// 
    /// # Returns
    ///
    /// A result indicating whether the update operation was successful.
    fn update(&self, connection: &mut Connection) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(self)?;

        if !self.exists(connection) {
            return Err(anyhow!("Model does not exist"));
        }

        redis::cmd("SET")
            .arg(self.key())
            .arg(serialized)
            .query::<()>(connection)?;

        Ok(())
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

        exists.unwrap_or(false)
    }

    /// Deletes the model from Redis without requiring an instance.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - The Redis connection to use for the delete operation.
    /// * `identifier` - The identifier of the model to delete.
    /// 
    /// # Returns
    /// 
    /// A boolean indicating whether the delete operation was successful.
    fn delete(connection: &mut Connection, identifier: &str) -> anyhow::Result<()> {
        redis::cmd("DEL")
            .arg(Self::fmt_key(identifier))
            .query::<bool>(connection)?;

        Ok(())
    }

    /// Formats the key for this model.
    fn fmt_key(identifier: &str) -> String;

    /// Returns the key for the model.
    fn key(&self) -> String;
}