use std::rc::Rc;

use crate::{
    client,
    delete_self,
    delete_user,
    login,
    rand_password,
    register,
    search_users,
    session,
    allow_reset_password,
    reset_password,
    try_admin_authorized_test,
    try_authorized_test
};
use crate::endpoint;
use chrono::Utc;
use reqwest::Client;
use reqwest::StatusCode;
use anyhow::{Result, anyhow};

#[tokio::test]
async fn test_get_session_not_logged_in() -> Result<()> {
    let client = client()?;
    let response = client.get(endpoint!("/api/users/session")).send().await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

#[tokio::test]
async fn test_get_session_logged_in() -> Result<()> {
    let client = client()?;

    try_authorized_test(client.clone(), |token| async move {
        let (status, json) = session(client, Some(token.as_str())).await?;
        assert_eq!(status, StatusCode::OK);
        assert!(!json.unwrap().user.is_admin);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_no_duplicate_users() -> Result<()> {
    const PASSWORD: &str = "goodpass2189389";

    let client = client()?;

    let status = register(client.clone(), "test", "Test User", PASSWORD).await?;
    assert_eq!(status, StatusCode::CREATED);

    let status = register(client.clone(), "test", "Tester", PASSWORD).await?;
    assert_eq!(status, StatusCode::CONFLICT);

    // Delete the user
    let (status, token, _) = login(client.clone(), "test", PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    let status = delete_self(client.clone(), token.unwrap().as_str(), PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_invalid_username_is_rejected() -> Result<()> {
    let client = client()?;

    match register(client, "test<script></script>", "Test User", "goodpass2189389").await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_invalid_name_is_rejected() -> Result<()> {
    let client = client()?;

    match register(client, "test", "Test User123", "goodpass2189389").await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_invalid_password_is_rejected() -> Result<()> {
    let client = client()?;

    match register(client, "test", "Test User", "").await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_cannot_search_user_if_not_admin() -> Result<()> {
    let client = client()?;

    try_authorized_test(client.clone(), |token| async move {
        let (status, _) = search_users(client, token.as_str(), "test").await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_delete_user_if_not_admin() -> Result<()> {
    let client = client()?;

    try_authorized_test(client.clone(), |token| async move {
        let status = delete_user(client, 1, token.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_search_users() -> Result<()> {
    const USER_COUNT: usize = 4;

    let mut cleanup = Vec::<(String, String)>::with_capacity(USER_COUNT);
    let client = client()?;

    for i in 0..USER_COUNT {
        let username = format!("search-test-{}", i);
        let name = "Search Test User";
        let password = rand_password();

        match register(client.clone(), username.as_str(), name, password.as_str()).await {
            Ok(status) if status == StatusCode::CREATED => (),
            Ok(status) => return Err(anyhow!("Unexpected register status code: {}", status)),
            Err(error) => return Err(error),
        }

        cleanup.push((username, password));
    }

    let client_clone = client.clone();
    try_admin_authorized_test(client_clone.clone(), |token| async move {
        let (status, users) = search_users(client_clone, token.as_str(), "search-test").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(users.unwrap().len(), USER_COUNT);

        Ok(())
    }).await?;

    // Login to each user and delete them
    for (username, password) in cleanup.iter() {
        let client_clone = client.clone();

        let (status, token, _) = login(client_clone.clone(), username, password).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_self(client_clone, password, token.unwrap().as_str()).await?;
        assert_eq!(status, StatusCode::OK);
    }

    Ok(())
}

#[tokio::test]
async fn test_reset_password() -> Result<()> {
    let client = client()?;

    const USERNAME: &str = "reset-password-user";
    const ORIGINAL_PASSWORD: &str = "resetpass69420";
    const NEW_PASSWORD: &str = "newpass69420";

    // Create a dummy user
    let status = register(client.clone(), USERNAME, "Reset Password", ORIGINAL_PASSWORD).await?;
    assert_eq!(status, StatusCode::CREATED);

    // Login as the dummy user
    let (status, auth_token, json) = login(client.clone(), USERNAME, ORIGINAL_PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    let dummy_user_id = json.unwrap().user.id;

    let mut reset_token = None;

    // Give the dummy user ability to reset their password as admin
    try_admin_authorized_test(client.clone(), |token| async move {
        let result = allow_reset_password(&client, dummy_user_id, token.as_str()).await?;
        assert_eq!(result.0, StatusCode::OK);

        reset_token = Some(result.1.unwrap().reset_token);

        Ok(())
    }).await?;

    // Reset the dummy user's password
    let status = reset_password(client.clone(), USERNAME, NEW_PASSWORD, reset_token.unwrap().as_str(), auth_token.unwrap().as_str()).await?;
    assert_eq!(status, StatusCode::OK);

    // Ensure old password is rejected
    let (status, _, _) = login(client.clone(), USERNAME, ORIGINAL_PASSWORD).await?;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Ensure new password is accepted
    let (status, auth_token, _) = login(client.clone(), USERNAME, NEW_PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    // Cleanup
    let status = delete_self(client, NEW_PASSWORD, auth_token.unwrap().as_str()).await?;
    assert_eq!(status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_admin_cannot_delete_admin() -> Result<()> {
    let client = client()?;

    try_admin_authorized_test(&client, || async {
        let (status, json) = session(&client).await?;
        assert_eq!(status, StatusCode::OK);

        let admin_id = json.unwrap().user.id;

        let status = delete_user(&client, admin_id).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        // Assert the acccount still exists
        let (status, _) = session(&client).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}