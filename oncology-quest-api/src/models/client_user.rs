use super::user::User;
use crate::utilities::parsable::{Username, Name};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct ClientUser {
    pub id: i32,
    pub username: Username,
    pub name: Name,
    pub is_admin: bool,
    pub login_count: i32
}

impl ClientUser {
    pub async fn text_search_as_map(pool: &PgPool, query: &str, limit: i64) -> anyhow::Result<HashMap<i32, Self>> {
        let users = sqlx::query_as!(
            Self,
            r#"
            SELECT users.id, users.username, users.name, users.is_admin, users.login_count
            FROM users
            WHERE (username ILIKE $1 OR name ILIKE $1)
            AND is_admin = FALSE
            LIMIT $2;
            "#,
            format!("%{}%", query),
            limit
        )
        .fetch_all(pool)
        .await?;

        let map = users
            .into_iter()
            .map(|result| { (result.id, result) })
            .collect::<HashMap<_, _>>();

        Ok(map)
    }
}

impl From<User> for ClientUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            username: user.username().to_owned(),
            name: user.name().to_owned(),
            is_admin: user.is_admin(),
            login_count: user.login_count()
        }
    }
}