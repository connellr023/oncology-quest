use crate::{
    client,
    try_authorized_test,
    register,
    login,
    delete_self,
    logout,
    session,
    search_users,
    delete_user
};
use crate::endpoint;
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
async fn test_get_session_logged_in() -> Result<()> {
    let (client, _) = client()?;
    try_authorized_test(&client, || async {
        let (status, _) = session(&client, None).await?;
        assert_eq!(status, StatusCode::OK);
    
        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_logout() -> Result<()> {
    let (client, _) = client()?;
    
    let status = register(&client, "logout-user", "Logout User", "logout@test.com", "whatthesigma").await?;
    assert_eq!(status, StatusCode::CREATED);
    
    let status = login(&client, "logout-user", "whatthesigma").await?;
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
    let status = login(&client, "logout-user", "whatthesigma").await?;
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