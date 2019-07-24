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
mod bugzilla;

// debug usage:
// RUST_BACKTRACE=1 cargo run
fn main() {
  let mut config_file = fs::File::open("config.yml").expect("unable to open config file");
  let mut config_text = String::new();
  config_file.read_to_string(&mut config_text).expect("unable to read config file");
  let config = &yaml_rust::YamlLoader::load_from_str(&config_text).unwrap()[0];

  // fetch previously stashed events from github gist
  let mut events = github::get_gist_events(
    config["github"]["events_gist"]["id"].as_str().unwrap().to_string(),
    config["github"]["events_gist"]["username"].as_str().unwrap().to_string(),
    config["github"]["events_gist"]["filename"].as_str().unwrap().to_string()
  );
  /*
  let bugMapCache: std::collections::HashMap<u32, DateTime<Utc>> = events
    .filter(|ref event| event.id.starts_with("Bugzilla_"))
    .map()
    .iter()
    .cloned()
    .collect();
  */
  // grab all pages of github events for each configured github username
  for github_username in config["github"]["usernames"].as_vec().unwrap().iter().map(|ref u| u.as_str().unwrap().to_string()) {
    let latest_stored_event_index = events.iter().position(|ref x| x.id.starts_with("GitHub_") && x.user == github_username).unwrap();
    let user_events = github::get_user_events(github_username, events[latest_stored_event_index].id.to_string());
    for user_event in user_events {
      // add or overwrite event
      events.retain(|ref e| e.id != user_event.id);
      events.push(user_event);
    }
  }

  for bugzilla_username in config["bugzilla"]["usernames"].as_vec().unwrap().iter().map(|ref u| u.as_str().unwrap().to_string()) {
    let user_events = bugzilla::get_user_events(bugzilla_username);
    for user_event in user_events {
      // add or overwrite event
      events.retain(|ref e| e.id != user_event.id);
      events.push(user_event);
    }
  }

  // sort events by date (newest first) and publish to github gist
  events.sort_by(|a, b| b.date.cmp(&a.date));
  let json_events = serde_json::to_string_pretty(&events).unwrap();
  github::update_gist_file(
    config["github"]["events_gist"]["id"].as_str().unwrap().to_string(),
    config["github"]["events_gist"]["description"].as_str().unwrap().to_string(),
    config["github"]["events_gist"]["filename"].as_str().unwrap().to_string(),
    json_events);
}