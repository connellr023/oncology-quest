use crate::{
    client,
    try_authorized_test,
    try_admin_authorized_test,
    create_rotation,
    delete_rotation
};
use anyhow::Result;
use reqwest::StatusCode;

#[tokio::test]
async fn test_non_admin_cannot_create_rotation() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, _) = create_rotation(&client_clone, "Test Rotation", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_invalid_rotation_name_is_rejected() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_admin_authorized_test(&client, |jwt| async move {
        let (status, _) = create_rotation(&client_clone, "<h1>Sneaky</h1>", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_create_and_delete_rotation() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();
    
    try_admin_authorized_test(&client, |jwt| async move {
        let (status, json) = create_rotation(&client_clone, "Test Rotation", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let status = delete_rotation(&client_clone, json.unwrap().rotation_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}