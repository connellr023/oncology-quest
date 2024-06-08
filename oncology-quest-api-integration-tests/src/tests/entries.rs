use crate::{
    client,
    create_rotation,
    delete_rotation,
    create_supertask,
    update_supertask,
    delete_supertask,
    create_task,
    update_task,
    delete_task,
    create_subtask,
    update_subtask,
    delete_subtask,
    get_entries,
    session,
    try_admin_authorized_test,
    try_authorized_test
};
use anyhow::Result;
use chrono::Utc;
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
async fn test_cannot_update_entry_if_not_admin() -> Result<()> {
    let (client, _) = client()?;

    try_authorized_test(&client, || async {
        let status = update_subtask(&client, 0, "Updated Subtask").await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let status = update_task(&client, 0, "Updated Task").await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let status = update_supertask(&client, 0, "Updated Supertask").await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_delete_entry_if_not_admin() -> Result<()> {
    let (client, _) = client()?;

    try_authorized_test(&client, || async {
        let status = delete_subtask(&client, 0).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let status = delete_task(&client, 0).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        let status = delete_supertask(&client, 0).await?;
        assert_eq!(status, StatusCode::FORBIDDEN);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_create_entry_without_rotation() -> Result<()> {
    let (client, _) = client()?;

    try_admin_authorized_test(&client, || async {
        let (status, _) = create_supertask(&client, "Test Supertask", 999).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        let (status, _) = create_task(&client, "Test Task", 0, 999).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        let (status, _) = create_subtask(&client, "Test Subtask", 0, 999).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_create_and_delete_entry() -> Result<()> {
    let (client, _) = client()?;

    try_admin_authorized_test(&client, || async {
        let (status, json) = create_rotation(&client, "Test Rotation").await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;

        let (status, supertask_id) = create_supertask(&client, "Test Supertask", rotation_id).await?;
        assert_eq!(status, StatusCode::CREATED);
        let supertask_id = supertask_id.unwrap();

        let (status, task_id) = create_task(&client, "Test Task", rotation_id, supertask_id).await?;
        assert_eq!(status, StatusCode::CREATED);
        let task_id = task_id.unwrap();

        let (status, subtask_id) = create_subtask(&client, "Test Subtask", rotation_id, task_id).await?;
        assert_eq!(status, StatusCode::CREATED);

        // Delete from bottom up (even though not necassary)
        let status = delete_subtask(&client, subtask_id.unwrap()).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_task(&client, task_id).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_supertask(&client, supertask_id).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_rotation(&client, rotation_id).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_get_entries_caching() -> Result<()> {
    let (client, _) = client()?;
    let mut rotation_id = -1;

    try_admin_authorized_test(&client, || async {
        let (status, json) = create_rotation(&client, "Test Rotation For Cache").await?;
        assert_eq!(status, StatusCode::CREATED);

        rotation_id = json.unwrap().rotation_id;

        let (status, _) = create_supertask(&client, "Test Supertask", rotation_id).await?;
        assert_eq!(status, StatusCode::CREATED);

        Ok(())
    }).await?;

    try_authorized_test(&client, || async {
        let (status, json) = session(&client, None).await?;
        let json = json.unwrap();
        assert_eq!(status, StatusCode::OK);
        assert!(json.rotations.contains_key(&rotation_id));
        
        let (status, json) = get_entries(&client, rotation_id, None).await?;
        assert_eq!(status, StatusCode::OK);
        assert!(json.unwrap().0.len() > 0);

        let (status, json) = get_entries(&client, rotation_id, Some(Utc::now())).await?;
        assert_eq!(status, StatusCode::NOT_MODIFIED);
        assert!(json.is_none());

        let (status, _) = session(&client, None).await?;
        assert_eq!(status, StatusCode::OK);

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
async fn test_update_entry() -> Result<()> {
    let (client, _) = client()?;

    try_admin_authorized_test(&client, || async {
        let (status, json) = create_rotation(&client, "Test Rotation For Update").await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;

        let (status, supertask_id) = create_supertask(&client, "Test Supertask", rotation_id).await?;
        assert_eq!(status, StatusCode::CREATED);
        let supertask_id = supertask_id.unwrap();

        let (status, task_id) = create_task(&client, "Test Task", rotation_id, supertask_id).await?;
        assert_eq!(status, StatusCode::CREATED);
        let task_id = task_id.unwrap();

        let (status, subtask_id) = create_subtask(&client, "Test Subtask", rotation_id, task_id).await?;
        assert_eq!(status, StatusCode::CREATED);

        let status = update_subtask(&client, subtask_id.unwrap(), "Updated Subtask").await?;
        assert_eq!(status, StatusCode::OK);

        let status = update_task(&client, task_id, "Updated Task").await?;
        assert_eq!(status, StatusCode::OK);

        let status = update_supertask(&client, supertask_id, "Updated Supertask").await?;
        assert_eq!(status, StatusCode::OK);
        
        let status = delete_rotation(&client, rotation_id).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}