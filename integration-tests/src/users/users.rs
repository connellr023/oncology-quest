use crate::prelude::*;

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
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, json) = session(&client_clone, Some(jwt.as_str())).await?;

        assert_eq!(status, StatusCode::OK);
        assert!(!json.unwrap().user.is_admin);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_logout() -> Result<()> {
    let client = client()?;

    let status = register(&client, "logout-user", "Logout User", "whatthesigma", ACCESS_CODE).await?;
    assert_eq!(status, StatusCode::CREATED);

    let (status, _, jwt) = login(&client, "logout-user", "whatthesigma").await?;
    assert_eq!(status, StatusCode::OK);

    // Check that the session is active
    let (status, _) = session(&client, Some(jwt.unwrap().as_str())).await?;
    assert_eq!(status, StatusCode::OK);

    // Check that we get nothing if the token is not provided
    let (status, _) = session(&client, None).await?;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Log back in
    let (status, _, jwt) = login(&client, "logout-user", "whatthesigma").await?;
    assert_eq!(status, StatusCode::OK);

    // Delete account
    let status = delete_self(&client, "whatthesigma", jwt.unwrap().as_str()).await?;
    assert_eq!(status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_no_duplicate_users() -> Result<()> {
    const PASSWORD: &str = "goodpass2189389";

    let client = client()?;

    let status = register(&client, "test", "Test User", PASSWORD, ACCESS_CODE).await?;
    assert_eq!(status, StatusCode::CREATED);

    let status = register(&client, "test", "Tester", PASSWORD, ACCESS_CODE).await?;
    assert_eq!(status, StatusCode::CONFLICT);

    // Delete the user
    let (status, _, jwt) = login(&client, "test", PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    let status = delete_self(&client, PASSWORD, jwt.unwrap().as_str()).await?;
    assert_eq!(status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_invalid_username_is_rejected() -> Result<()> {
    let client = client()?;

    match register(&client, "test<script></script>", "Test User", "goodpass2189389", ACCESS_CODE).await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_invalid_name_is_rejected() -> Result<()> {
    let client = client()?;

    match register(&client, "test", "Test User123", "goodpass2189389", ACCESS_CODE).await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_invalid_password_is_rejected() -> Result<()> {
    let client = client()?;

    match register(&client, "test", "Test User", "", ACCESS_CODE).await {
        Ok(status) if status == StatusCode::BAD_REQUEST => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn test_cannot_search_user_if_not_admin() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, _) = search_users(&client_clone, "test", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_delete_user_if_not_admin() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let status = delete_user(&client_clone, 1, jwt.as_str()).await?;
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

        match register(&client, username.as_str(), name, password.as_str(), ACCESS_CODE).await {
            Ok(status) if status == StatusCode::CREATED => (),
            Ok(status) => return Err(anyhow!("Unexpected register status code: {}", status)),
            Err(error) => return Err(error),
        }

        cleanup.push((username, password));
    }

    let client_clone = client.clone();

    try_admin_authorized_test(&client, |jwt| async move {
        let (status, users) = search_users(&client_clone, "search-test", jwt.as_str()).await?;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(users.unwrap().len(), USER_COUNT);

        Ok(())
    }).await?;

    // Login to each user and delete them
    for (username, password) in cleanup.iter() {
        let (status, _, jwt) = login(&client, username, password).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_self(&client, password, jwt.unwrap().as_str()).await?;
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
    let status = register(&client, USERNAME, "Reset Password", ORIGINAL_PASSWORD, ACCESS_CODE).await?;
    assert_eq!(status, StatusCode::CREATED);

    // Login as the dummy user
    let (status, json, _) = login(&client, USERNAME, ORIGINAL_PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    let dummy_user_id = json.unwrap().user.id;
    let client_clone = client.clone();

    // Give the dummy user ability to reset their password as admin
    let reset_token = try_admin_authorized_test(&client, |jwt| async move {
        let result = allow_reset_password(&client_clone, dummy_user_id, jwt.as_str()).await?;
        assert_eq!(result.0, StatusCode::OK);

        let token = result.1.unwrap().reset_token;
        Ok(token)
    }).await?;

    // Reset the dummy user's password
    let status = reset_password(&client, USERNAME, NEW_PASSWORD, reset_token.as_str()).await?;
    assert_eq!(status, StatusCode::OK);

    // Ensure old password is rejected
    let (status, _, _) = login(&client, USERNAME, ORIGINAL_PASSWORD).await?;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Ensure new password is accepted
    let (status, _, jwt) = login(&client, USERNAME, NEW_PASSWORD).await?;
    assert_eq!(status, StatusCode::OK);

    // Cleanup
    let status = delete_self(&client, NEW_PASSWORD, jwt.unwrap().as_str()).await?;
    assert_eq!(status, StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_admin_cannot_delete_admin() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_admin_authorized_test(&client, |jwt| async move {
        let (status, json) = session(&client_clone, Some(jwt.as_str())).await?;
        assert_eq!(status, StatusCode::OK);

        let admin_id = json.unwrap().user.id;

        let status = delete_user(&client_clone, admin_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        // Assert the acccount still exists
        let (status, _) = session(&client_clone, Some(jwt.as_str())).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_invalid_access_code_is_rejected() -> Result<()> {
    let client = client()?;

    match register(&client, "test", "Test User", "testpass123", "invalid-access-code").await {
        Ok(status) if status == StatusCode::UNAUTHORIZED => (),
        Ok(status) => return Err(anyhow!("Unexpected status code: {}", status)),
        Err(error) => return Err(error),
    }

    Ok(())
}
