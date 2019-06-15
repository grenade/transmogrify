#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate restson;
use restson::{RestClient};
use std::env;

mod github;

// debug usage:
// RUST_BACKTRACE=1 cargo run -- grenade
fn main() {
  let mut github_api = RestClient::new(github::API_URL).unwrap();
  let github_activity: github::Activity = github_api.get(env::args().nth(1).unwrap()).unwrap();
  println!("{:?}", github_activity);
}