use crate::prelude::*;

async fn setup_subtask(client: &reqwest::Client, rotation_id: i32, jwt: &str) -> Result<i32> {
    let (status, id) = create_supertask(&client, "Test Supertask", rotation_id, jwt).await?;
    assert_eq!(status, StatusCode::CREATED);

    let (status, id) = create_task(&client, "Test Task", rotation_id, id.unwrap(), jwt).await?;
    assert_eq!(status, StatusCode::CREATED);

    let (status, id) = create_subtask(&client, "Test Subtask", rotation_id, id.unwrap(), jwt).await?;
    assert_eq!(status, StatusCode::CREATED);

    Ok(id.unwrap())
}

#[tokio::test]
async fn test_admin_cannot_have_tasks() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_admin_authorized_test(&client, |jwt| async move {
        let (status, json) = create_rotation(&client_clone, "Test Rotation Comments", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;
        let subtask_id = setup_subtask(&client_clone, rotation_id, jwt.as_str()).await?;

        let (status, _) = create_user_task(&client_clone, rotation_id, subtask_id, false, "Hi", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let status = delete_rotation(&client_clone, rotation_id, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::OK);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_invalid_task_comment_is_rejected() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    let (rotation_id, subtask_id) = try_admin_authorized_test(&client, |jwt| async move {
        let (status, json) = create_rotation(&client_clone, "Test Rotation Comments", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;
        let subtask_id = setup_subtask(&client_clone, rotation_id, jwt.as_str()).await?;

        Ok((rotation_id, subtask_id))
    }).await?;

    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, _) = create_user_task(&client_clone, rotation_id, subtask_id, false, "<h1>XSS</h1>", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

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
async fn test_no_duplicate_tasks() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    let (rotation_id, subtask_id) = try_admin_authorized_test(&client, |jwt| async move {
        let (status, json) = create_rotation(&client_clone, "Test Rotation Comments", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;
        let subtask_id = setup_subtask(&client_clone, rotation_id, jwt.as_str()).await?;

        Ok((rotation_id, subtask_id))
    }).await?;
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, _) = create_user_task(&client_clone, rotation_id, subtask_id, false, "Hi", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let (status, _) = create_user_task(&client_clone, rotation_id, subtask_id, false, "Hi", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CONFLICT);

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
async fn test_cannot_create_task_on_nonexistent_subtask() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    let rotation_id = try_admin_authorized_test(&client, |jwt| async move {
        let (status, json) = create_rotation(&client_clone, "Test Rotation Comments", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::CREATED);

        let rotation_id = json.unwrap().rotation_id;

        Ok(rotation_id)
    }).await?;

    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, _) = create_user_task(&client_clone, rotation_id, 999, true, "Comment", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

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
async fn test_cannot_create_task_on_nonexistent_rotation() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, _) = create_user_task(&client_clone, 999, 999, true, "Comment", jwt.as_str()).await?;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        Ok(())
    }).await?;

    Ok(())
}

#[tokio::test]
async fn test_regular_user_cannot_get_other_tasks() -> Result<()> {
    let client = client()?;
    let client_clone = client.clone();

    try_authorized_test(&client, |jwt| async move {
        let (status, _) = get_user_tasks(&client_clone, 1, 1, jwt.as_str()).await?;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        Ok(())
    }).await?;

    Ok(())
}
