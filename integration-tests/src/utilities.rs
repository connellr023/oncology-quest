use crate::prelude::*;

pub fn rand_username() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>()
}

pub fn rand_password() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect::<String>()
}

pub fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

pub async fn try_authorized_test<F, T, V>(client: &Client, callback: T) -> Result<V>
where
    F: Future<Output = Result<V>>,
    T: FnOnce(String) -> F,
{
    let username = rand_username();
    let name = "Test User";
    let password = rand_password();

    match register(client, username.as_str(), name, password.as_str(), ACCESS_CODE).await {
        Ok(status) if status == StatusCode::CREATED => (),
        Ok(status) => return Err(anyhow!("Unexpected register status code: {}", status)),
        Err(error) => return Err(error),
    }

    #[allow(unused)]
    let mut jwt = None;

    match login(client, username.as_str(), password.as_str()).await {
        Ok((status, _, token)) if status == StatusCode::OK => { jwt = token },
        Ok((status, _, _)) => return Err(anyhow!("Unexpected login status code: {}", status)),
        Err(error) => return Err(error),
    }

    let jwt = jwt.unwrap();
    let result = callback(jwt.clone()).await?;

    match delete_self(client, password.as_str(), jwt.as_str()).await {
        Ok(status) if status == StatusCode::OK => (),
        Ok(status) => return Err(anyhow!("Unexpected delete status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(result)
}

pub async fn try_admin_authorized_test<F, T, V>(client: &Client, callback: T) -> Result<V>
where
    F: Future<Output = Result<V>>,
    T: FnOnce(String) -> F,
{
    #[allow(unused)]
    let mut jwt = None;

    match login(client, ADMIN_USERNAME, ADMIN_PASSWORD).await {
        Ok((status, _, token)) if status == StatusCode::OK => { jwt = token },
        Ok((status, _, _)) => return Err(anyhow!("Unexpected admin login status code: {}", status)),
        Err(error) => return Err(error),
    }

    let jwt = jwt.unwrap();
    let result = callback(jwt.clone()).await?;

    Ok(result)
}
