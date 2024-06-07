use crate::client;
use crate::endpoint;
use std::future::Future;
use reqwest::Client;
use reqwest::StatusCode;
use serde_json::json;
use anyhow::{Result, anyhow};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn rand_username() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>()
}

fn rand_password() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect::<String>()
}

fn rand_email() -> String {
    format!("{}@{}.com", rand_username(), rand_username())
}

pub async fn register(client: &Client, username: &str, name: &str, email: &str, password: &str) -> Result<StatusCode> {
    let response = client.post(endpoint!("/api/users/register"))
        .json(&json!({
            "username": username,
            "name": name,
            "email": email,
            "password": password
        }))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn login(client: &Client, username: &str, password: &str) -> Result<StatusCode> {
    let response = client.post(endpoint!("/api/users/login"))
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .await?;

    response.cookies().for_each(|cookie| {
        println!("Name: {}", cookie.name());
        println!("Path: {}", cookie.path().unwrap_or_default());
        println!("Value: {}", cookie.value());
    });

    Ok(response.status())
}

pub async fn delete_self(client: &Client, password: &str) -> Result<StatusCode> {
    let response = client.delete(endpoint!("/api/users/delete-self"))
        .json(&json!({
            "password": password
        }))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn try_authorized_test<F>(client: &Client, callback: impl FnOnce() -> F) -> Result<()>
where F: Future<Output = Result<()>> {
    let username = rand_username();
    let name = "Test User";
    let email = rand_email();
    let password = rand_password();

    match register(client, username.as_str(), name, email.as_str(), password.as_str()).await {
        Ok(status) if status == StatusCode::CREATED => (),
        Ok(status) => return Err(anyhow!("Unexpected register status code: {}", status)),
        Err(error) => return Err(error),
    }

    match login(client, username.as_str(), password.as_str()).await {
        Ok(status) if status == StatusCode::OK => (),
        Ok(status) => return Err(anyhow!("Unexpected login status code: {}", status)),
        Err(error) => return Err(error),
    }

    callback().await?;

    match delete_self(client, password.as_str()).await {
        Ok(status) if status == StatusCode::OK => (),
        Ok(status) => return Err(anyhow!("Unexpected delete status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_session_not_logged_in() -> Result<()> {
    let (client, _) = client()?;
    let response = client.get(endpoint!("/api/users/session")).send().await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

#[tokio::test]
async fn test_get_session_logged_in() -> Result<()> {
    let (client, _) = client()?;
    try_authorized_test(&client, || async {
        let response = client.get(endpoint!("/api/users/session")).send().await?;
        assert_eq!(response.status(), StatusCode::OK);
    
        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_register_login_delete() -> Result<()> {
    let (client, _) = client()?;
    try_authorized_test(&client, || async { Ok(()) }).await?;

    Ok(())
}

#[tokio::test]
async fn test_invalid_username_is_rejected() -> Result<()> {
    let (client, _) = client()?;

    match register(&client, "test<script></script>", "Test User", "test@test.com", "goodpass2189389").await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_invalid_name_is_rejected() -> Result<()> {
    let (client, _) = client()?;

    match register(&client, "test", "Test User123", "test@test.com", "goodpass2189389").await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_invalid_email_is_rejected() -> Result<()> {
    let (client, _) = client()?;

    match register(&client, "test", "Test User", "notanemail", "goodpass2189389").await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_invalid_password_is_rejected() -> Result<()> {
    let (client, _) = client()?;

    match register(&client, "test", "Test User", "notanemail", "").await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}