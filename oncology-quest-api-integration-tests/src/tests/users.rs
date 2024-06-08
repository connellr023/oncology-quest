use crate::{
    client,
    delete_self,
    delete_user,
    login,
    logout,
    rand_email,
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
    let (client, _) = client()?;
    let response = client.get(endpoint!("/api/users/session")).send().await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

#[tokio::test]
async fn test_get_session_logged_in_invalid_cache() -> Result<()> {
    let (client, _) = client()?;

    try_authorized_test(&client, || async {
        let (status, json) = session(&client, None).await?;
        assert_eq!(status, StatusCode::OK);
        assert!(json.unwrap().tasks.is_some());
    
        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_get_session_logged_in_valid_cache() -> Result<()> {
    let (client, _) = client()?;

    try_authorized_test(&client, || async {
        let (status, json) = session(&client, Some(Utc::now())).await?;
        assert_eq!(status, StatusCode::OK);
        assert!(json.unwrap().tasks.is_none());
    
        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_logout() -> Result<()> {
    let (client, _) = client()?;
    
    let status = register(&client, "logout-user", "Logout User", "logout@test.com", "whatthesigma").await?;
    assert_eq!(status, StatusCode::CREATED);
    
    let (status, _) = login(&client, "logout-user", "whatthesigma").await?;
    assert_eq!(status, StatusCode::OK);

    // Check that the session is active
    let (status, _) = session(&client, None).await?;
    assert_eq!(status, StatusCode::OK);

    // Logout
    logout(&client).await?;

    // Check that the session is inactive
    let (status, _) = session(&client, None).await?;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Log back in
    let (status, _) = login(&client, "logout-user", "whatthesigma").await?;
    assert_eq!(status, StatusCode::OK);

    // Delete account
    let status = delete_self(&client, "whatthesigma").await?;
    assert_eq!(status, StatusCode::OK);

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

#[tokio::test]
async fn test_cannot_search_user_if_not_admin() -> Result<()> {
    let (client, _) = client()?;
    try_authorized_test(&client, || async {
        let (status, _) = search_users(&client, "test").await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_delete_user_if_not_admin() -> Result<()> {
    let (client, _) = client()?;
    try_authorized_test(&client, || async {
        let status = delete_user(&client, 1).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_search_users() -> Result<()> {
    const USER_COUNT: usize = 4;

    let mut cleanup = Vec::<(String, String)>::with_capacity(USER_COUNT);
    let (client, _) = client()?;

    for i in 0..USER_COUNT {
        let username = format!("search-test-{}", i);
        let name = "Search Test User";
        let email = rand_email();
        let password = rand_password();

        match register(&client, username.as_str(), name, email.as_str(), password.as_str()).await {
            Ok(status) if status == StatusCode::CREATED => (),
            Ok(status) => return Err(anyhow!("Unexpected register status code: {}", status)),
            Err(error) => return Err(error),
        }

        cleanup.push((username, password));
    }

    try_admin_authorized_test(&client, || async {
        let (status, users) = search_users(&client, "search-test").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(users.unwrap().len(), USER_COUNT);

        Ok(())
    }).await?;

    // Login to each user and delete them
    for (username, password) in cleanup.iter() {
        let (status, _) = login(&client, username, password).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_self(&client, password).await?;
        assert_eq!(status, StatusCode::OK);
    }

    Ok(())
}

#[tokio::test]
async fn test_reset_password() -> Result<()> {
    let (client, _) = client()?;

    const USERNAME: &str = "reset-password-user";
    const ORIGINAL_PASSWORD: &str = "resetpass69420";
    const NEW_PASSWORD: &str = "newpass69420";

    // Create a dummy user
    let status = register(&client, USERNAME, "Reset Password", "resetpass@skibidi.net", ORIGINAL_PASSWORD).await?;
    assert_eq!(status, StatusCode::CREATED);

    // Login as the dummy user
    let (status, json) = login(&client, USERNAME, ORIGINAL_PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    let dummy_user_id = json.unwrap().user.id;

    // Logout
    let status = logout(&client).await?;
    assert_eq!(status, StatusCode::OK);

    // Give the dummy user ability to reset their password as admin
    try_admin_authorized_test(&client, || async {
        let status = allow_reset_password(&client, dummy_user_id).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    // Reset the dummy user's password
    let status = reset_password(&client, USERNAME, NEW_PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    // Ensure old password is rejected
    let (status, _) = login(&client, USERNAME, ORIGINAL_PASSWORD).await?;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Ensure new password is accepted
    let (status, _) = login(&client, USERNAME, NEW_PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    // Cleanup
    let status = delete_self(&client, NEW_PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_admin_cannot_delete_admin() -> Result<()> {
    let (client, _) = client()?;

    try_admin_authorized_test(&client, || async {
        let (status, json) = session(&client, None).await?;
        assert_eq!(status, StatusCode::OK);

        let admin_id = json.unwrap().user.id;

        // Query will go through, but it won't actually delete the admin
        let status = delete_user(&client, admin_id).await?;
        assert_eq!(status, StatusCode::OK);

        // Assert the acccount still exists
        let (status, _) = session(&client, None).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}