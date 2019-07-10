extern crate chrono;
extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate yaml_rust;
mod entity;
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

  // grab all pages of github events for each configured github username
  for github_username in config["github"]["usernames"].clone() {
    let url = format!("{}/users/{}/events", github::API_URL, github_username.as_str().unwrap().to_string());
    for page in 1..(github::get_last_page_number(url.clone()) + 1) {
      let github_events: Vec<github::Event> = serde_json::from_str(reqwest::get(&format!("{}?page={}", url, page)).unwrap().text().unwrap().as_str()).unwrap();
      for github_event in github_events {
        println!("{:?}", github_event);
      }
    }
  }
}