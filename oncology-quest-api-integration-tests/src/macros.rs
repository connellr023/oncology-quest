#[macro_export]
macro_rules! endpoint {
    ($route:expr) => {
        format!("http://api:8000{}", $route)
    }
}

#[macro_export]
macro_rules! delete_entry_fn {
    ($entry_level:literal, $fn_name:ident) => {
        pub async fn $fn_name(client: &Client, entry_id: i32, jwt: &str) -> Result<StatusCode> {
            let response = client.delete(endpoint!(format!("/api/entries/{}/delete", $entry_level)))
                .json(&json!({ "entryId": entry_id }))
                .header(AUTHORIZATION, jwt)
                .send()
                .await?;

            Ok(response.status())
        }
    };
}

#[macro_export]
macro_rules! update_entry_fn {
    ($entry_level:literal, $fn_name:ident) => {
        pub async fn $fn_name(client: &Client, entry_id: i32, title: &str, jwt: &str) -> Result<StatusCode> {
            let response = client.patch(endpoint!(format!("/api/entries/{}/update", $entry_level)))
                .json(&json!({
                    "entryId": entry_id,
                    "title": title
                }))
                .header(AUTHORIZATION, jwt)
                .send()
                .await?;

            Ok(response.status())
        }
    };
}