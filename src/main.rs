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
  env,
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

  let mut events: Vec<entity::Event> = Vec::new();

  // grab all pages of github events for each configured github username
  for github_username in config["github"]["usernames"].clone() {
    let url = format!("{}/users/{}/events", github::API_URL, github_username.as_str().unwrap().to_string());
    for page in 1..(github::get_last_page_number(url.clone()) + 1) {
      let gh_user = env::var("GH_USER").unwrap().to_string();
      let gh_pass = env::var("GH_PASS").unwrap().to_string();
      let mut response = reqwest::Client::new().get(&format!("{}?page={}", url, page)).basic_auth(gh_user.clone(), Some(gh_pass.clone())).send().unwrap();
      let github_events: Vec<github::Event> = serde_json::from_str(response.text().unwrap().as_str()).unwrap();
      for github_event in github_events {
        println!("page {} - {:?}", page, &github_event.action);
        println!("{:?}", &github_event);
        let e = match github_event.action.as_ref() {
          "PushEvent" => entity::Event{
            id: github_event.id.to_string(),
            action: format!("Github{}", &github_event.action),
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
                suffix: Some(commit.message.lines().next().unwrap().to_string()),
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
            id: github_event.id.to_string(),
            action: format!("Github{}", &github_event.action),
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
            id: github_event.id.to_string(),
            action: format!("Github{}", &github_event.action),
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
            id: github_event.id.to_string(),
            action: format!("Github{}", &github_event.action),
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
            id: github_event.id.to_string(),
            action: format!("Github{}", &github_event.action),
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
  }
  events.sort_by(|a, b| b.date.cmp(&a.date));
  let json_events = serde_json::to_string_pretty(&events).unwrap();
  let json_events_path = format!("/home/grenade/git/gist/4882bcabb5b7d0d31e67153839998819/grenade-events.json");
  fs::write(&json_events_path, &json_events).expect("unable to write json file");
  println!("{} updated",  json_events_path);
}