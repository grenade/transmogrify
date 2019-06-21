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

fn extract_page(input: &str) -> Option<&u32> {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"page=([0-9]*)").unwrap();
  }
  RE.captures(input).and_then(|cap| {
    cap.name("page").map(|page| page.parse::<u32>())
  })
}

pub fn get_last_page_number(url: String) -> u32 {
  let response = reqwest::get(&url).expect("failed to send request");
  if response.status().is_success() {
    println!("{}", response.status());


    let last_page_link_header = response.headers().get(reqwest::header::LINK).unwrap().to_str().unwrap().split(",").last().unwrap()
    let re = Regex::new(r"page=(?P<page>[0-9]*)").unwrap();
    let cap = match re.captures(&last_page_link_header) {
      Some(data) => data,
      None => panic!("failed to match page number in link header")
    };
    let page = match cap.name("idno") {
      Some(data) => Some(match data.parse::<u32>() {
        Ok(data) => data,
        Err(err) => panic!("died in u32 parse")
      }),
      None => None
    };
    println!("{:?}", extract_page());
    return 1;
  } else {
    return 0;
  }
  //for header in response.headers().iter() {
    //println!("{}: {}", header.name(), header.value_string());
  //  println!("{:?}", header);
  //}
}