use chrono::{DateTime, Utc};
use sqlx::{FromRow, Postgres};

#[derive(Debug, FromRow)]
pub struct Domain {
    id: i32,
    name: String,
    last_updated: DateTime<Utc>
}

impl Domain {
    pub async fn fetch(pool: &sqlx::Pool<Postgres>, primary_key: i32) -> anyhow::Result<Self> {
        let domain = sqlx::query_as!(
            Domain,
            r#"
            SELECT * FROM domains WHERE id = $1;
            "#,
            primary_key
        )
        .fetch_one(pool)
        .await?;

        Ok(domain)
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