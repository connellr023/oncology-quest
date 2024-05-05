use super::{tasks::UserTaskEntries, user::User, model::Model};
use serde::{Serialize, Deserialize};
use redis::Connection;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClientUser {
    pub username: String,
    pub name: String,
    pub email: String,
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
        let keys = redis::cmd("KEYS")
            .arg(pattern)
            .query::<Vec<String>>(connection)?;

        for key in keys {
            let user_encoding = redis::cmd("GET")
                .arg(key)
                .query::<String>(connection)?;
            let user = serde_json::from_str::<User>(&user_encoding)?;
            let client_user = ClientUser::from(user);

            users.push(client_user);
        }

        users.retain(|user| user.is_admin == false);
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