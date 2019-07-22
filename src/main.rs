extern crate chrono;
extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate yaml_rust;
use std::{
  fs,
  io::prelude::*
};

mod entity;
mod github;

// debug usage:
// RUST_BACKTRACE=1 cargo run
fn main() {
  let mut config_file = fs::File::open("config.yml").expect("unable to open config file");
  let mut config_text = String::new();
  config_file.read_to_string(&mut config_text).expect("unable to read config file");
  let config = &yaml_rust::YamlLoader::load_from_str(&config_text).unwrap()[0];

  let mut events: Vec<entity::Event> = Vec::new();

  // grab all pages of github events for each configured github username
  for github_username in config["github"]["usernames"].clone() {
    events.append(&mut github::get_user_events(github_username.as_str().unwrap().to_string()));
  }
  events.sort_by(|a, b| b.date.cmp(&a.date));
  let json_events = serde_json::to_string_pretty(&events).unwrap();
  github::update_gist_file(
    "4882bcabb5b7d0d31e67153839998819".to_string(),
    "things grenade did recently".to_string(),
    "grenade-events.json".to_string(),
    json_events);
}