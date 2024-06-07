#![crate_name = "oncology_quest_api_integration_tests"]
#![allow(unused_imports)]
#![allow(dead_code)]

mod tests;
mod macros;

use std::sync::Arc;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use anyhow::Result;

fn main() {
    println!("Endpoint Macro: {}", endpoint!("/api/..."));
    println!("Run tests with `cargo test`");
}

pub fn client() -> Result<(Client, Arc<CookieStoreMutex>)> {
    let cookie_store = Arc::new(CookieStoreMutex::default());
    let client = Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()?;

    Ok((client, cookie_store))
}