use super::phantom_data::Synced;
use crate::models::rotation::Rotation;
use std::{collections::HashMap, sync::RwLock};
use anyhow::{anyhow, Result};

type CachedRotations = Option<HashMap<i32, Rotation<Synced>>>;

pub struct MemoryCache {
    cached_rotations_lock: RwLock<CachedRotations>
}

impl MemoryCache {
    pub fn new() -> Self {
        Self { cached_rotations_lock: RwLock::new(None) }
    }

    pub fn get_rotations_as_clone(&self) -> Result<CachedRotations> {
        let read_lock = self.cached_rotations_lock
            .try_read()
            .map_err(|_| anyhow!("Failed to acquire read lock on cached rotations."))?;

        Ok((*read_lock).clone())
    }

    pub fn set_rotations(&self, rotations: CachedRotations) -> Result<()> {
        let mut write_lock = self.cached_rotations_lock
            .try_write()
            .map_err(|_| anyhow!("Failed to acquire write lock on cached rotations."))?;

        *write_lock = rotations;

        Ok(())
    }

    pub fn insert_rotation(&self, rotation: Rotation<Synced>) -> Result<()> {
        let mut write_lock = self.cached_rotations_lock
            .try_write()
            .map_err(|_| anyhow!("Failed to acquire write lock on cached rotations."))?;

        if let Some(cached_rotations) = write_lock.as_mut() {
            cached_rotations.insert(rotation.id(), rotation);
        }

        Ok(())
    }

    pub fn delete_rotation(&self, rotation_id: i32) -> Result<()> {
        let mut write_lock = self.cached_rotations_lock
            .try_write()
            .map_err(|_| anyhow!("Failed to acquire write lock on cached rotations."))?;

        if let Some(cached_rotations) = write_lock.as_mut() {
            cached_rotations.remove(&rotation_id);
        }

        Ok(())
    }
}
