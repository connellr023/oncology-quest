#![crate_name = "oncology_quest_api_integration_tests"]
#![allow(unused_imports)]
#![allow(dead_code)]

mod entries;
mod rotations;
mod tasks;
mod users;
mod macros;
mod responses;
mod utilities;
mod prelude;

use reqwest::Client;
use anyhow::Result;

// These are assumed to be set in the environment variables of the endpoint server
pub const ACCESS_CODE: &str = "testcode";
pub const ADMIN_USERNAME: &str = "admin";
pub const ADMIN_PASSWORD: &str = "complexpass123";

fn main() {
    println!("Endpoint Macro: {}", endpoint!("/api/..."));
    println!("Run tests with `cargo test`");
}

#[inline(always)]
pub fn client() -> Result<Client> {
    Ok(Client::builder().build()?)
}
