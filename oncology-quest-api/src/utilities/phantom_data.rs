/// Phantom data type to represent that this model is synced with the database
pub struct InDatabase;

/// Phantom data type to represent that this model is not synced with the database
pub struct Unknown;

/// Phantom data for base users that are not admins and are synced with the database
pub struct Regular;

/// Phantom data for users that are admins and are synced with the database
pub struct Admin;