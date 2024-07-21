use crate::prelude::*;

pub async fn get_owned_user_tasks(client: &Client, rotation_id: i32) -> Result<(StatusCode, Option<GetUserTasksResponse>)> {
    let response = client.get(endpoint!(format!("/api/tasks/{}", rotation_id)).as_str())
        .send()
        .await?;

    Ok((response.status(), response.json().await.ok()))
}

pub async fn get_user_tasks(client: &Client, rotation_id: i32, user_id: i32, jwt: &str) -> Result<(StatusCode, Option<GetUserTasksResponse>)> {
    let response = client.get(endpoint!(format!("/api/tasks/{}/{}", rotation_id, user_id)).as_str())
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    Ok((response.status(), response.json().await.ok()))
}

pub async fn create_user_task(client: &Client, rotation_id: i32, subtask_id: i32, is_completed: bool, comment: &str, jwt: &str) -> Result<(StatusCode, Option<i32>)> {
    let response = client.post(endpoint!("/api/tasks/create"))
        .json(&json!({
            "subtaskId": subtask_id,
            "isCompleted": is_completed,
            "rotationId": rotation_id,
            "comment": comment
        }))
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    let status_code = response.status();
    let json = response
        .json::<CreateUserTaskResponse>()
        .await
        .ok();

    Ok((status_code, json.map(|json| { json.id })))
}

pub async fn update_user_task(client: &Client, user_task_id: i32, is_completed: bool, comment: &str) -> Result<StatusCode> {
    let response = client.patch(endpoint!("/api/tasks/update"))
        .json(&json!({
            "id": user_task_id,
            "isCompleted": is_completed,
            "comment": comment
        }))
        .send()
        .await?;

    Ok(response.status())
}
