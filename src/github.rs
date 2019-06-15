//use chrono::{DateTime, Utc};
use serde_json::{Value};
use restson::{RestPath,Error};

pub const API_URL: &str = "https://api.github.com";

#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum Activity {
  Array(Vec<Value>)
}

impl RestPath<String> for Activity {
  fn get_path(github_username: String) -> Result<String,Error> {
    Ok(format!("users/{}/events", github_username))
  }
}