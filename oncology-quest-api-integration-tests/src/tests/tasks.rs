use crate::{
    client, create_rotation, create_subtask, create_supertask, create_task, create_user_task, delete_rotation, get_user_tasks, try_admin_authorized_test, try_authorized_test, update_user_task
};
use anyhow::Result;
use reqwest::StatusCode;

async fn setup_subtask(client: &reqwest::Client, rotation_id: i32) -> Result<i32> {
    let (status, id) = create_supertask(&client, "Test Supertask", rotation_id).await?;
    assert_eq!(status, StatusCode::CREATED);

    let (status, id) = create_task(&client, "Test Task", rotation_id, id.unwrap()).await?;
    assert_eq!(status, StatusCode::CREATED);

    let (status, id) = create_subtask(&client, "Test Subtask", rotation_id, id.unwrap()).await?;
    assert_eq!(status, StatusCode::CREATED);

    Ok(id.unwrap())
}

#[tokio::test]
async fn test_admin_cannot_have_tasks() -> Result<()> {
    let (client, _) = client()?;

    try_admin_authorized_test(&client, || async {
        let (status, json) = create_rotation(&client, "Test Rotation Comments").await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;
        let subtask_id = setup_subtask(&client, rotation_id).await?;

        let (status, _) = create_user_task(&client, rotation_id, subtask_id, false, "Hi").await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let status = delete_rotation(&client, rotation_id).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_invalid_task_comment_is_rejected() -> Result<()> {
    let (client, _) = client()?;
    let mut rotation_id = -1;
    let mut subtask_id = -1;

    try_admin_authorized_test(&client, || async {
        let (status, json) = create_rotation(&client, "Test Rotation Comments").await?;
        assert_eq!(status, StatusCode::CREATED);

        rotation_id = json.unwrap().rotation_id;
        subtask_id = setup_subtask(&client, rotation_id).await?;

        Ok(())
    }).await?;

    try_authorized_test(&client, || async {
        let (status, _) = create_user_task(&client, rotation_id, subtask_id, false, "<h1>XSS</h1>").await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        Ok(())
    }).await?;

    try_admin_authorized_test(&client, || async {
        let status = delete_rotation(&client, rotation_id).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_no_duplicate_tasks() -> Result<()> {
    let (client, _) = client()?;
    let mut rotation_id = -1;
    let mut subtask_id = -1;

    try_admin_authorized_test(&client, || async {
        let (status, json) = create_rotation(&client, "Test Rotation Comments").await?;
        assert_eq!(status, StatusCode::CREATED);

        rotation_id = json.unwrap().rotation_id;
        subtask_id = setup_subtask(&client, rotation_id).await?;

        Ok(())
    }).await?;

    try_authorized_test(&client, || async {
        let (status, _) = create_user_task(&client, rotation_id, subtask_id, false, "Hi").await?;
        assert_eq!(status, StatusCode::CREATED);

        let (status, _) = create_user_task(&client, rotation_id, subtask_id, false, "Hi").await?;
        assert_eq!(status, StatusCode::CONFLICT);

        Ok(())
    }).await?;

    try_admin_authorized_test(&client, || async {
        let status = delete_rotation(&client, rotation_id).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_create_task_on_nonexistent_subtask() -> Result<()> {
    let (client, _) = client()?;
    let mut rotation_id = -1;

    try_admin_authorized_test(&client, || async {
        let (status, json) = create_rotation(&client, "Test Rotation Comments").await?;
        assert_eq!(status, StatusCode::CREATED);

        rotation_id = json.unwrap().rotation_id;

        Ok(())
    }).await?;

    try_authorized_test(&client, || async {
        let (status, _) = create_user_task(&client, rotation_id, 999, true, "Comment").await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        Ok(())
    }).await?;

    try_admin_authorized_test(&client, || async {
        let status = delete_rotation(&client, rotation_id).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_create_task_on_nonexistent_rotation() -> Result<()> {
    let (client, _) = client()?;

    try_authorized_test(&client, || async {
        let (status, _) = create_user_task(&client, 999, 999, true, "Comment").await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_regular_user_cannot_get_other_tasks() -> Result<()> {
    let (client, _) = client()?;
    
    try_authorized_test(&client, || async {
        let (status, _) = get_user_tasks(&client, 1, 1).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}