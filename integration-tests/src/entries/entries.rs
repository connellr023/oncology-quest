use crate::prelude::*;

#[tokio::test]
async fn test_cannot_create_entry_if_not_admin() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, _) = create_supertask(&client_clone, "Test Supertask", 0, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let (status, _) = create_task(&client_clone, "Test Task", 0, 0, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let (status, _) = create_subtask(&client_clone, "Test Subtask", 0, 0, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_update_entry_if_not_admin() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let status = update_subtask(&client_clone, 0, "Updated Subtask", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let status = update_task(&client_clone, 0, "Updated Task", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let status = update_supertask(&client_clone, 0, "Updated Supertask", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_delete_entry_if_not_admin() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let status = delete_subtask(&client_clone, 0, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let status = delete_task(&client_clone, 0, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let status = delete_supertask(&client_clone, 0, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_cannot_create_entry_without_rotation() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_admin_authorized_test(&client, |jwt| async move {
        let (status, _) = create_supertask(&client_clone, "Test Supertask", 999, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        let (status, _) = create_task(&client_clone, "Test Task", 0, 999, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        let (status, _) = create_subtask(&client_clone, "Test Subtask", 0, 999, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_create_and_delete_entry() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_admin_authorized_test(&client, |jwt| async move {
        let (status, json) = create_rotation(&client_clone, "Test Rotation", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;

        let (status, supertask_id) = create_supertask(&client_clone, "Test Supertask", rotation_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);
        let supertask_id = supertask_id.unwrap();

        let (status, task_id) = create_task(&client_clone, "Test Task", rotation_id, supertask_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);
        let task_id = task_id.unwrap();

        let (status, subtask_id) = create_subtask(&client_clone, "Test Subtask", rotation_id, task_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        // Delete from bottom up (even though not necassary)
        let status = delete_subtask(&client_clone, subtask_id.unwrap(), jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_task(&client_clone, task_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_supertask(&client_clone, supertask_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_rotation(&client_clone, rotation_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_get_entries_caching() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    let rotation_id = try_admin_authorized_test(&client, |jwt| async move {
        let (status, json) = create_rotation(&client_clone, "Test Rotation For Cache", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;

        let (status, _) = create_supertask(&client_clone, "Test Supertask", rotation_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        Ok(rotation_id)
    }).await?;

    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, json) = session(&client_clone, Some(jwt.as_str())).await?;
        let json = json.unwrap();
        assert_eq!(status, StatusCode::OK);
        assert!(json.rotations.contains_key(&rotation_id));

        let (status, json) = get_entries(&client_clone, rotation_id, None, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);
        assert!(json.unwrap().0.len() > 0);

        let (status, json) = get_entries(&client_clone, rotation_id, Some(Utc::now()), jwt.as_str()).await?;
        assert_eq!(status, StatusCode::NOT_MODIFIED);
        assert!(json.is_none());

        let (status, _) = session(&client_clone, Some(jwt.as_str())).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    let client_clone = client.clone();

    try_admin_authorized_test(&client, |jwt| async move {
        let status = delete_rotation(&client_clone, rotation_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_update_entry() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_admin_authorized_test(&client, |jwt| async move {
        let (status, json) = create_rotation(&client_clone, "Test Rotation For Update", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;

        let (status, supertask_id) = create_supertask(&client_clone, "Test Supertask", rotation_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);
        let supertask_id = supertask_id.unwrap();

        let (status, task_id) = create_task(&client_clone, "Test Task", rotation_id, supertask_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);
        let task_id = task_id.unwrap();

        let (status, subtask_id) = create_subtask(&client_clone, "Test Subtask", rotation_id, task_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let status = update_subtask(&client_clone, subtask_id.unwrap(), "Updated Subtask", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        let status = update_task(&client_clone, task_id, "Updated Task", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        let status = update_supertask(&client_clone, supertask_id, "Updated Supertask", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        let status = delete_rotation(&client_clone, rotation_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}
