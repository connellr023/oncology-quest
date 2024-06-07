use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRotationResponse {
    pub rotation_id: i32,
    pub last_updated: DateTime<Utc>
}