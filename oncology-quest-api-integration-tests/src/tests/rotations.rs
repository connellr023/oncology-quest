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
    let (client, _) = client()?;

    try_authorized_test(&client, || async {
        let (status, _) = create_rotation(&client, "Test Rotation").await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_invalid_rotation_name_is_rejected() -> Result<()> {
    let (client, _) = client()?;

    try_admin_authorized_test(&client, || async {
        let (status, _) = create_rotation(&client, "<h1>Sneaky</h1>").await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_create_and_delete_rotation() -> Result<()> {
    let (client, _) = client()?;
    
    try_admin_authorized_test(&client, || async {
        let (status, json) = create_rotation(&client, "Test Rotation").await?;
        assert_eq!(status, StatusCode::CREATED);

        let status = delete_rotation(&client, json.unwrap().rotation_id).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}