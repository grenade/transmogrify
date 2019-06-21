extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate restson;
extern crate yaml_rust;

use restson::{RestClient};
use std::{
  fs,
  io::prelude::*
};

mod github;

// debug usage:
// RUST_BACKTRACE=1 cargo run
fn main() {
  let mut config_file = fs::File::open("config.yml").expect("unable to open config file");
  let mut config_text = String::new();
  config_file.read_to_string(&mut config_text).expect("unable to read config file");
  let config = &yaml_rust::YamlLoader::load_from_str(&config_text).unwrap()[0];

  let mut github_api = RestClient::new(github::API_URL).unwrap();
  let query = vec![("page", "2")];
  for github_username in config["github"]["usernames"].clone() {
    let github_activity: github::Activity = github_api.get_with(github_username.as_str().unwrap().to_string(), &query).unwrap();
    println!("{:?}", github_activity);
  }
}