#![crate_name = "oncology_quest_api_integration_tests"]
#![allow(unused_imports)]
#![allow(dead_code)]

mod tests;
mod macros;
mod responses;

use std::{sync::Arc, future::Future, collections::HashMap};
use chrono::{DateTime, Utc};
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use anyhow::{Result, anyhow};
use reqwest::StatusCode;
use responses::{CreateEntryResponse, CreateRotationResponse, CreateUserTaskResponse, EntryStructure, SearchResultUser, SearchUserResponse, UserSessionResponse};
use serde_json::json;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    println!("Endpoint Macro: {}", endpoint!("/api/..."));
    println!("Run tests with `cargo test`");
}

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

fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

pub async fn session(client: &Client, task_cache_timestamp: Option<DateTime<Utc>>) -> Result<(StatusCode, Option<UserSessionResponse>)> {
    let response = match task_cache_timestamp {
        Some(timestamp) => client.get(endpoint!(format!("/api/users/session?taskCacheTimestamp={}", format_timestamp(timestamp))))
            .send()
            .await?,
        None => client.get(endpoint!("/api/users/session"))
            .send()
            .await?,
    };

    Ok((response.status(), response.json().await.ok()))
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

pub async fn login(client: &Client, username: &str, password: &str) -> Result<(StatusCode, Option<UserSessionResponse>)> {
    let response = client.post(endpoint!("/api/users/login"))
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .await?;

    Ok((response.status(), response.json().await.ok()))
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

pub async fn logout(client: &Client) -> Result<StatusCode> {
    let response = client.post(endpoint!("/api/users/logout"))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn search_users(client: &Client, query: &str) -> Result<(StatusCode, Option<SearchUserResponse>)> {
    let endpoint = format!("{}/{}", endpoint!("/api/users/search"), query);

    let response = client.get(endpoint)
        .send()
        .await?;

    Ok((response.status(), response.json().await.ok()))
}

pub async fn delete_user(client: &Client, user_id: i32) -> Result<StatusCode> {
    let response = client.delete(endpoint!("/api/users/delete-other-user"))
        .json(&json!({
            "userId": user_id
        }))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn reset_password(client: &Client, username: &str, password: &str) -> Result<StatusCode> {
    let response = client.post(endpoint!("/api/users/reset-password"))
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn allow_reset_password(client: &Client, user_id: i32) -> Result<StatusCode> {
    let response = client.patch(endpoint!("/api/users/allow-reset-password"))
        .json(&json!({ "userId": user_id }))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn try_authorized_test<F, T>(client: &Client, callback: T) -> Result<()>
where
    F: Future<Output = Result<()>>,
    T: FnOnce() -> F,
{
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
        Ok((status, _)) if status == StatusCode::OK => (),
        Ok((status, _)) => return Err(anyhow!("Unexpected login status code: {}", status)),
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

pub async fn create_rotation(client: &Client, name: &str) -> Result<(StatusCode, Option<CreateRotationResponse>)> {
    let response = client.post(endpoint!("/api/rotations/create"))
        .json(&json!({ "name": name }))
        .send()
        .await?;

    Ok((response.status(), response.json().await.ok()))
}

pub async fn delete_rotation(client: &Client, rotation_id: i32) -> Result<StatusCode> {
    let response = client.delete(endpoint!("/api/rotations/delete"))
        .json(&json!({ "rotationId": rotation_id }))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn create_supertask(client: &Client, title: &str, rotation_id: i32) -> Result<(StatusCode, Option<i32>)> {
    let response = client.post(endpoint!("/api/entries/supertasks/create"))
        .json(&json!({
            "title": title,
            "rotationId": rotation_id
        }))
        .send()
        .await?;

    let status_code = response.status();
    let json = response
        .json::<CreateEntryResponse>()
        .await
        .ok();

    Ok((status_code, json.map(|json| { json.entry_id })))
}

pub async fn create_task(client: &Client, title: &str, rotation_id: i32, supertask_id: i32) -> Result<(StatusCode, Option<i32>)> {
    let response = client.post(endpoint!("/api/entries/tasks/create"))
        .json(&json!({
            "title": title,
            "rotationId": rotation_id,
            "parentId": supertask_id
        }))
        .send()
        .await?;

    let status_code = response.status();
    let json = response
        .json::<CreateEntryResponse>()
        .await
        .ok();

    Ok((status_code, json.map(|json| { json.entry_id })))
}

pub async fn create_subtask(client: &Client, title: &str, rotation_id: i32, task_id: i32) -> Result<(StatusCode, Option<i32>)> {
    let response = client.post(endpoint!("/api/entries/subtasks/create"))
        .json(&json!({
            "title": title,
            "rotationId": rotation_id,
            "parentId": task_id
        }))
        .send()
        .await?;

    let status_code = response.status();
    let json = response
        .json::<CreateEntryResponse>()
        .await
        .ok();

    Ok((status_code, json.map(|json| { json.entry_id })))
}

delete_entry_fn!("supertasks", delete_supertask);
delete_entry_fn!("tasks", delete_task);
delete_entry_fn!("subtasks", delete_subtask);

update_entry_fn!("supertasks", update_supertask);
update_entry_fn!("tasks", update_task);
update_entry_fn!("subtasks", update_subtask);

pub async fn get_entries(client: &Client, rotation_id: i32, entries_cache_timestamp: Option<DateTime<Utc>>) -> Result<(StatusCode, Option<EntryStructure>)> {
    let response = match entries_cache_timestamp {
        Some(timestamp) => client.get(endpoint!(format!("/api/entries/{}?entriesCacheTimestamp={}", rotation_id, format_timestamp(timestamp))).as_str())
            .send()
            .await?,
        None => client.get(endpoint!(format!("/api/entries/{}", rotation_id)).as_str())
            .send()
            .await?,
    };

    Ok((response.status(), response.json().await.ok()))
}

pub async fn create_user_task(client: &Client, subtask_id: i32, is_completed: bool, comment: &str) -> Result<(StatusCode, Option<i32>)> {
    let response = client.post(endpoint!("/api/tasks/create"))
        .json(&json!({
            "subtaskId": subtask_id,
            "isCompleted": is_completed,
            "comment": comment
        }))
        .send()
        .await?;

    let status_code = response.status();
    let json = response
        .json::<CreateUserTaskResponse>()
        .await
        .ok();

    Ok((status_code, json.map(|json| { json.id })))
}

pub async fn update_user_task(client: &Client, user_task_id: i32, is_completed: bool, comment: &str) -> Result<StatusCode> {
    let response = client.patch(endpoint!("/api/tasks/update"))
        .json(&json!({
            "id": user_task_id,
            "isCompleted": is_completed,
            "comment": comment
        }))
        .send()
        .await?;

    Ok(response.status())
}

pub async fn try_admin_authorized_test<F, T>(client: &Client, callback: T) -> Result<()>
where
    F: Future<Output = Result<()>>,
    T: FnOnce() -> F,
{
    // Admin account is assumed to exist in the test database
    const ADMIN_USERNAME: &str = "admin";
    const ADMIN_PASSWORD: &str = "complexpass123";

    match login(client, ADMIN_USERNAME, ADMIN_PASSWORD).await {
        Ok((status, _)) if status == StatusCode::OK => (),
        Ok((status, _)) => return Err(anyhow!("Unexpected admin login status code: {}", status)),
        Err(error) => return Err(error),
    }

    callback().await?;

    match logout(client).await {
        Ok(status) if status == StatusCode::OK => (),
        Ok(status) => return Err(anyhow!("Unexpected admin logout status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

pub fn client() -> Result<(Client, Arc<CookieStoreMutex>)> {
    let cookie_store = Arc::new(CookieStoreMutex::default());
    let client = Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()?;

    Ok((client, cookie_store))
}