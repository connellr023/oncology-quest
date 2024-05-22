use super::{model::Model, tasks::UserTaskEntries, user::{User, USER_KEY_SET}};
use crate::utilities::parsable::{Username, Name, Email};
use serde::{Serialize, Deserialize};
use redis::Connection;

const MAX_SEARCH_RESULTS: u8 = 15;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClientUser {
    pub username: Username,
    pub name: Name,
    pub email: Email,
    pub tasks: UserTaskEntries,
    pub is_admin: bool
}

impl ClientUser {
    /// Searches for users based on the provided query.
    /// 
    /// # Arguments
    /// 
    /// * `connection` - A mutable reference to the Redis connection.
    /// * `query` - The query to search for.
    /// 
    /// # Returns
    /// 
    /// Returns a Result containing a vector of users that match the query. If an error occurs, it will be returned.
    pub fn text_search(connection: &mut Connection, query: &str) -> anyhow::Result<Vec<Self>> {
        let mut users = vec![];
        let pattern = User::fmt_key(format!("*{}*", query).as_str());
        let keys_iter = redis::cmd("SSCAN")
            .arg(USER_KEY_SET)
            .cursor_arg(0)
            .arg("MATCH")
            .arg(pattern)
            .arg("COUNT")
            .arg(MAX_SEARCH_RESULTS)
            .clone()
            .iter::<String>(connection)?;

        let mut pipe = redis::pipe();
        for key in keys_iter {
            pipe.cmd("GET").arg(key);
        }

        let users_encoding: Vec<String> = pipe.query(connection)?;
        for user_encoding in users_encoding {
            let user = serde_json::from_str::<User>(&user_encoding)?;
            let client_user = ClientUser::from(user);
            users.push(client_user);
        }

        users.retain(|user| !user.is_admin);
        Ok(users)
    }
}

impl From<User> for ClientUser {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            name: user.name,
            email: user.email,
            tasks: user.tasks,
            is_admin: user.is_admin
        }
    }
}