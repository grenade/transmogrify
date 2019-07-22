use chrono::{DateTime, Utc};
use regex::Regex;
use serde_json::json;
use std::env;
use entity;

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
  pub action: Option<String>,
  pub descripttion: Option<String>,
  pub master_branch: Option<String>,
  #[serde(rename = "ref")]
  pub git_ref: Option<String>,
  pub ref_type: Option<String>,
  pub commits: Option<Vec<Commit>>,
  pub comment: Option<Comment>,
  pub forkee: Option<Fork>,
  pub pull_request: Option<PullRequest>,
}

#[derive(Deserialize, Debug)]
pub struct Repo {
  pub id: u32,
  pub name: String,
  pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Comment {
  pub id: u32,
  pub body: String,
  pub url: String,
  pub html_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Fork {
  pub id: u32,
  pub name: String,
  pub full_name: String,
  pub url: String,
  pub html_url: String,
}

#[derive(Deserialize, Debug)]
pub struct PullRequest {
  pub id: u32,
  pub title: String,
  pub state: String,
  pub body: Option<String>,
  pub url: String,
  pub html_url: String,
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


pub fn get_user_events(github_username: String) -> Vec<entity::Event> {
  let mut events: Vec<entity::Event> = Vec::new();
  let url = format!("{}/users/{}/events", API_URL, github_username);
  for page in 1..(get_last_page_number(url.clone()) + 1) {
    let gh_user = env::var("GH_USER").unwrap().to_string();
    let gh_pass = env::var("GH_PASS").unwrap().to_string();
    let mut response = reqwest::Client::new().get(&format!("{}?page={}", url, page)).basic_auth(gh_user.clone(), Some(gh_pass.clone())).send().unwrap();
    let github_events: Vec<Event> = serde_json::from_str(response.text().unwrap().as_str()).unwrap();
    for github_event in github_events {
      println!("page {} - {:?}", page, &github_event.action);
      println!("{:?}", &github_event);
      let e = match github_event.action.as_ref() {
        "PushEvent" => entity::Event{
          id: format!("GitHub_{}", &github_event.id),
          action: format!("GitHub_{}", &github_event.action),
          date: github_event.created_at,
          title: entity::Element{
            definition: None,
            prefix: Some("git push to ".to_string()),
            url: Some(format!("https://github.com/{}/tree/{}", &github_event.repo.name, github_event.payload.git_ref.as_ref().unwrap().split("/").last().unwrap())),
            text: format!("{} {}", &github_event.repo.name, github_event.payload.git_ref.as_ref().unwrap().split("/").last().unwrap()),
            title: None,
            suffix: None,
          },
          subtitle: Some(entity::Element{
            definition: None,
            prefix: None,
            url: Some(format!("https://github.com/{}", &github_event.actor.display_login)),
            text: github_event.actor.display_login.clone(),
            title: None,
            suffix: Some(format!(" pushed {} commit{}", github_event.payload.commits.as_ref().unwrap().len(), (if github_event.payload.commits.as_ref().unwrap().len() == 1 { "" } else { "s" }))),
          }),
          body: Some(entity::Body{
            content: github_event.payload.commits.as_ref().unwrap().iter().map(|commit| entity::Element{
              definition: None,
              prefix: None,
              url: Some(format!("https://github.com/{}/commit/{}", &github_event.repo.name, &commit.sha)),
              text: format!("{}", &commit.sha[0..7]),
              title: Some(format!("commit {}", &commit.sha[0..7])),
              suffix: Some(format!(" {}", commit.message.lines().next().unwrap().to_string())),
            }).collect(),
            tag: entity::Tag::Icon
          }),
        },
        /*
        Event {
          actor: Actor {
            avatar_url: "https://avatars.githubusercontent.com/u/111819?",
            display_login: "grenade",
            gravatar_id: "",
            id: 111819,
            login: "grenade",
            url: "https://api.github.com/users/grenade"
          },
          created_at: 2019-07-08T12:34:10Z,
          id: "9966964627",
          org: None,
          payload: Payload {
            descripttion: None,
            master_branch: None,
            git_ref: None,
            ref_type: None,
            commits: None
          },
          action: "ForkEvent",
          repo: Repo {
            id: 71844754,
            name: "timwis/markdown-to-google-doc",
            url: "https://api.github.com/repos/timwis/markdown-to-google-doc"
          }
        }
        */
        "ForkEvent" => entity::Event{
          id: format!("GitHub_{}", &github_event.id),
          action: format!("GitHub_{}", &github_event.action),
          date: github_event.created_at,
          title: entity::Element{
            definition: None,
            prefix: None,
            url: None,
            text: github_event.action.clone(),
            title: None,
            suffix: None,
          },
          subtitle: Some(entity::Element{
            definition: None,
            prefix: None,
            url: None,
            text: github_event.action.clone(),
            title: None,
            suffix: None,
          }),
          body: Some(entity::Body{
            content: vec![
              entity::Element{
                definition: None,
                prefix: None,
                url: None,
                text: github_event.action.clone(),
                title: None,
                suffix: None,
              },
            ],
            tag: entity::Tag::Icon
          }),
        },
        "PullRequestEvent" => entity::Event{
          id: format!("GitHub_{}", &github_event.id),
          action: format!("GitHub_{}", &github_event.action),
          date: github_event.created_at,
          title: entity::Element{
            definition: None,
            prefix: Some("pull request ".to_string()),
            url: Some(format!("{}", github_event.payload.pull_request.as_ref().unwrap().html_url)),
            text: format!("{}", &github_event.payload.action.unwrap()),
            title: None,
            suffix: None,
          },
          subtitle: Some(entity::Element{
            definition: None,
            prefix: None,
            url: None,
            text: format!("{}", github_event.payload.pull_request.as_ref().unwrap().title),
            title: None,
            suffix: None,
          }),
          body: (if github_event.payload.pull_request.as_ref().unwrap().body.as_ref().unwrap() != "" {
            Some(entity::Body{
              content: vec![
                entity::Element{
                  definition: None,
                  prefix: None,
                  url: None,
                  text: format!("{}", github_event.payload.pull_request.as_ref().unwrap().body.as_ref().unwrap()),
                  title: None,
                  suffix: None,
                },
              ],
              tag: entity::Tag::Icon
            })
          } else {
            None
          }),
        },
        /*
        Event {
          actor: Actor { avatar_url: "https://avatars.githubusercontent.com/u/111819?", display_login: "grenade", gravatar_id: "", id: 111819, login: "grenade", url: "https://api.github.com/users/grenade" },
          created_at: 2019-07-09T11:17:20Z,
          id: "9974697518",
          org: None,
          payload: Payload {
            descripttion: None,
            master_branch: None,
            git_ref: None,
            ref_type: None,
            commits: None
          },
          action: "IssueCommentEvent",
          repo: Repo { id: 571770, name: "jgm/pandoc", url: "https://api.github.com/repos/jgm/pandoc" }
        }
        */
        "IssueCommentEvent" => entity::Event{
          id: format!("GitHub_{}", &github_event.id),
          action: format!("GitHub_{}", &github_event.action),
          date: github_event.created_at,
          title: entity::Element{
            definition: None,
            prefix: None,
            url: None,
            text: github_event.action.clone(),
            title: None,
            suffix: None,
          },
          subtitle: Some(entity::Element{
            definition: None,
            prefix: None,
            url: None,
            text: github_event.action.clone(),
            title: None,
            suffix: None,
          }),
          body: Some(entity::Body{
            content: vec![
              entity::Element{
                definition: None,
                prefix: None,
                url: None,
                text: github_event.action.clone(),
                title: None,
                suffix: None,
              },
            ],
            tag: entity::Tag::Icon
          }),
        },
        _ => entity::Event{
          id: format!("GitHub_{}", &github_event.id),
          action: format!("GitHub_{}", &github_event.action),
          date: github_event.created_at,
          title: entity::Element{
            definition: None,
            prefix: None,
            url: None,
            text: github_event.action.clone(),
            title: None,
            suffix: None,
          },
          subtitle: None,
          body: None,
        }
      };
      println!("{:?}", &e);
      events.push(e);
    }
  }
  return events;
}


pub fn update_gist_file(gist_id: String, gist_description: String, file_name: String, file_content: String) {
  let gh_user = env::var("GH_USER").unwrap().to_string();
  let gh_pass = env::var("GH_PASS").unwrap().to_string();
  let body = json!({
    "description": gist_description,
    "files": {
      file_name.clone(): {
        "content": file_content,
        "filename": file_name
      }
    }
  });
  let response = reqwest::Client::new().patch(&format!("{}/gists/{}", API_URL, gist_id))
    .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
    .body(body.to_string())
    .send().unwrap();
  println!("{}", response.status());
}