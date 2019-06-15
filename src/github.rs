use chrono::{DateTime, Utc};
//use serde_json::{Value};
use restson::{RestPath,Error};

pub const API_URL: &str = "https://api.github.com";

#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum Activity {
  Array(Vec<Event>)
}

#[derive(Deserialize, Debug)]
pub struct Actor {
    pub avatar_url: String,
    pub display_login: String,
    pub gravatar_id: String,
    pub id: u32,
    pub login: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub author: Author,
    pub message: String,
    pub sha: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub actor: Actor,
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub org: Option<Org>,
    pub payload: Payload,
    #[serde(rename = "type")]
    pub action: String,
    pub repo: Repo,
}

#[derive(Deserialize, Debug)]
pub struct Org {
    pub avatar_url: String,
    pub gravatar_id: String,
    pub id: u32,
    pub login: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub descripttion: Option<String>,
    pub master_branch: Option<String>,
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,
    pub ref_type: Option<String>,
    pub commits: Option<Vec<Commit>>,
}

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub id: u32,
    pub name: String,
    pub url: String,
}


impl RestPath<String> for Activity {
  fn get_path(github_username: String) -> Result<String,Error> {
    Ok(format!("users/{}/events", github_username))
  }
}