use crate::prelude::*;

pub async fn session(client: &Client, jwt: Option<&str>) -> Result<(StatusCode, Option<UserSessionResponse>)> {
    let response = match jwt {
        Some(jwt) => client.get(endpoint!("/api/users/session"))
            .header(AUTHORIZATION, jwt)
            .send()
            .await?,
        None => client.get(endpoint!("/api/users/session"))
            .send()
            .await?,
    };

    Ok((response.status(), response.json().await.ok()))
}

pub async fn register(client: &Client, username: &str, name: &str, password: &str, access_code: &str) -> Result<StatusCode> {
    let response = client.post(endpoint!("/api/users/register"))
        .json(&json!({
            "username": username,
            "name": name,
            "password": password,
            "accessCode": access_code
        }))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn login(client: &Client, username: &str, password: &str) -> Result<(StatusCode, Option<UserSessionResponse>, Option<String>)> {
    let response = client.post(endpoint!("/api/users/login"))
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .await?;

    let jwt = response.headers().get(AUTHORIZATION)
        .map(|header| header.to_str().ok())
        .flatten()
        .map(|header| header.to_string());

    Ok((response.status(), response.json().await.ok(), jwt))
}

pub async fn delete_self(client: &Client, password: &str, jwt: &str) -> Result<StatusCode> {
    let response = client.delete(endpoint!("/api/users/delete-self"))
        .json(&json!({
            "password": password
        }))
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    Ok(response.status())
}

pub async fn search_users(client: &Client, query: &str, jwt: &str) -> Result<(StatusCode, Option<SearchUserResponse>)> {
    let endpoint = format!("{}/{}", endpoint!("/api/users/search"), query);

    let response = client.get(endpoint)
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    Ok((response.status(), response.json().await.ok()))
}

pub async fn delete_user(client: &Client, user_id: i32, jwt: &str) -> Result<StatusCode> {
    let response = client.delete(endpoint!("/api/users/delete-other-user"))
        .json(&json!({
            "userId": user_id
        }))
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    Ok(response.status())
}

pub async fn reset_password(client: &Client, username: &str, password: &str, token: &str) -> Result<StatusCode> {
    let response = client.post(endpoint!("/api/users/reset-password"))
        .json(&json!({
            "username": username,
            "password": password,
            "resetToken": token
        }))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn allow_reset_password(client: &Client, user_id: i32, jwt: &str) -> Result<(StatusCode, Option<AllowResetPasswordResponse>)> {
    let response = client.patch(endpoint!("/api/users/allow-reset-password"))
        .json(&json!({ "userId": user_id }))
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    Ok((response.status(), response.json().await.ok()))
}
