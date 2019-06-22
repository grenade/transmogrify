extern crate chrono;
extern crate regex;
extern crate reqwest;
extern crate restson;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate yaml_rust;

use restson::{
  RestClient,
  RestPath
};
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
  
  for github_username in config["github"]["usernames"].clone() {
    let last_page = github::get_last_page_number(format!("{}/{}", github::API_URL, github::Activity::get_path(github_username.as_str().unwrap().to_string()).unwrap()));
    println!("last_page: {:?}", last_page);
    for i in 0..last_page {
      let page = format!("{}", (i + 1));
      println!("fetch page: {:?}", page);
      let query = vec![("page", &page[..])];
      let github_activity: github::Activity = github_api.get_with(github_username.as_str().unwrap().to_string(), query.as_slice()).unwrap();
      println!("{:?}", github_activity);
    }
  }
}