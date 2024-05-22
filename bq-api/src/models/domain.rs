use chrono::{DateTime, Utc};
use sqlx::{FromRow, Postgres};
use serde::Serialize;

#[derive(Serialize, Debug, FromRow)]
pub struct Domain {
    id: i32,
    name: String,
    last_updated: DateTime<Utc>
}

impl Domain {
    pub async fn fetch_all(pool: &sqlx::Pool<Postgres>) -> anyhow::Result<Box<[Self]>> {
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

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn last_updated(&self) -> DateTime<Utc> {
        self.last_updated
    }
}