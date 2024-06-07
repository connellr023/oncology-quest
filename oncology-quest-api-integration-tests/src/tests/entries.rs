use crate::{
    client,
    create_supertask,
    create_task,
    create_subtask,
    session,
    try_authorized_test
};
use anyhow::Result;
use reqwest::StatusCode;

#[tokio::test]
async fn test_cannot_create_entry_if_not_admin() -> Result<()> {
    let (client, _) = client()?;

    try_authorized_test(&client, || async {
        let (status, _) = create_supertask(&client, "Test Supertask", 0).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, _) = create_task(&client, "Test Task", 0, 0).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let (status, _) = create_subtask(&client, "Test Subtask", 0, 0).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_create_entry_without_rotation() -> Result<()> {
    Ok(())
}