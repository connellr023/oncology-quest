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

fn main() {
    println!("Endpoint Macro: {}", endpoint!("/api/..."));
    println!("Run tests with `cargo test`");
}

#[inline(always)]
pub fn client() -> Result<Client> {
    Ok(Client::builder().build()?)
}
