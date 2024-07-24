use crate::prelude::*;

pub async fn create_rotation(client: &Client, name: &str, jwt: &str) -> Result<(StatusCode, Option<CreateRotationResponse>)> {
    let response = client.post(endpoint!("/api/rotations/create"))
        .json(&json!({ "name": name }))
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    Ok((response.status(), response.json().await.ok()))
}

pub async fn delete_rotation(client: &Client, rotation_id: i32, jwt: &str) -> Result<StatusCode> {
    let response = client.delete(endpoint!("/api/rotations/delete"))
        .json(&json!({ "rotationId": rotation_id }))
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    Ok(response.status())
}
