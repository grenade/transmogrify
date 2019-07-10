use chrono::{DateTime, Utc};
use regex::Regex;

pub const API_URL: &str = "https://api.github.com";

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

pub fn get_last_page_number(url: String) -> u32 {
  let response = reqwest::get(&url).expect("failed to send request");
  if response.status().is_success() {
    println!("{}", response.status());
    let last_page_link_header = response.headers().get(reqwest::header::LINK).unwrap().to_str().unwrap().split(",").last().unwrap();
    let re = Regex::new(r"page=(?:([0-9]+))").unwrap();
    let captures = match re.captures(&last_page_link_header) {
      Some(m) => m,
      None => panic!("failed to match last page number in link header")
    };
    return captures.get(1).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
  } else {
    return 0;
  }
}