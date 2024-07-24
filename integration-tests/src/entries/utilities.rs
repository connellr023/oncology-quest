use crate::prelude::*;

pub async fn get_entries(client: &Client, rotation_id: i32, entries_cache_timestamp: Option<DateTime<Utc>>, jwt: &str) -> Result<(StatusCode, Option<EntryStructure>)> {
    let response = match entries_cache_timestamp {
        Some(timestamp) => client.get(endpoint!(format!("/api/entries/{}?entriesCacheTimestamp={}", rotation_id, format_timestamp(timestamp))).as_str())
            .header(AUTHORIZATION, jwt)
            .send()
            .await?,
        None => client.get(endpoint!(format!("/api/entries/{}", rotation_id)).as_str())
            .header(AUTHORIZATION, jwt)
            .send()
            .await?,
    };

    Ok((response.status(), response.json().await.ok()))
}

pub async fn create_supertask(client: &Client, title: &str, rotation_id: i32, jwt: &str) -> Result<(StatusCode, Option<i32>)> {
    let response = client.post(endpoint!("/api/entries/supertasks/create"))
        .json(&json!({
            "title": title,
            "rotationId": rotation_id
        }))
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    let status_code = response.status();
    let json = response
        .json::<CreateEntryResponse>()
        .await
        .ok();

    Ok((status_code, json.map(|json| { json.entry_id })))
}

pub async fn create_task(client: &Client, title: &str, rotation_id: i32, supertask_id: i32, jwt: &str) -> Result<(StatusCode, Option<i32>)> {
    let response = client.post(endpoint!("/api/entries/tasks/create"))
        .json(&json!({
            "title": title,
            "rotationId": rotation_id,
            "parentId": supertask_id
        }))
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    let status_code = response.status();
    let json = response
        .json::<CreateEntryResponse>()
        .await
        .ok();

    Ok((status_code, json.map(|json| { json.entry_id })))
}

pub async fn create_subtask(client: &Client, title: &str, rotation_id: i32, task_id: i32, jwt: &str) -> Result<(StatusCode, Option<i32>)> {
    let response = client.post(endpoint!("/api/entries/subtasks/create"))
        .json(&json!({
            "title": title,
            "rotationId": rotation_id,
            "parentId": task_id
        }))
        .header(AUTHORIZATION, jwt)
        .send()
        .await?;

    let status_code = response.status();
    let json = response
        .json::<CreateEntryResponse>()
        .await
        .ok();

    Ok((status_code, json.map(|json| { json.entry_id })))
}

delete_entry_fn!("supertasks", delete_supertask);
delete_entry_fn!("tasks", delete_task);
delete_entry_fn!("subtasks", delete_subtask);

update_entry_fn!("supertasks", update_supertask);
update_entry_fn!("tasks", update_task);
update_entry_fn!("subtasks", update_subtask);
