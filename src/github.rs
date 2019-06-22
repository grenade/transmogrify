use chrono::{DateTime, Utc};
//use serde_json::{Value};
use regex::Regex;
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

pub fn get_last_page_number(url: String) -> u32 {
  let response = reqwest::get(&url).expect("failed to send request");
  println!("url: {}", url);
  println!("response: {}", response.status());
  if response.status().is_success() {
    let last_page_link_header = response.headers().get(reqwest::header::LINK).unwrap().to_str().unwrap().split(",").last().unwrap();
    let re = Regex::new(r"page=(?P<page>[0-9]*)").unwrap();
    let captures = match re.captures(&last_page_link_header) {
      Some(data) => data,
      None => panic!("failed to match page number regex in link header")
    };
    let page = match captures.name("page") {
      Some(data) => Some(match data.as_str().parse::<u32>() {
        Ok(data) => data,
        Err(_) => panic!("failed to parse page number")
      }),
      None => None
    };
    println!("{:?}", page);
    return page.unwrap();
  } else {
    return 0;
  }
}