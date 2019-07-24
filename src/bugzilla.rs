use chrono::{DateTime, Utc};
use entity;

pub const API_URL: &str = "https://bugzilla.mozilla.org/rest";


#[derive(Deserialize, Debug)]
pub struct BugListResponse {
  pub bugs: Vec<Bug>,
}

#[derive(Deserialize, Debug)]
pub struct Bug {
  pub id: u32,
  pub last_change_time: DateTime<Utc>,
  pub creation_time: DateTime<Utc>,
  pub priority: String,
  pub severity: String,
  pub assigned_to_detail: Option<User>,
  pub creator_detail: User,
  pub comment_count: u32,
  pub votes: u32,
  pub resolution: String,
  pub status: String,
  #[serde(rename = "type")]
  pub action: String,
  pub summary: String,
  pub op_sys: String,
  pub platform: String,

  pub product: String,
  pub component: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
  pub id: u32,
  pub email: String,
  pub nick: String,
  pub real_name: String,
}

#[derive(Deserialize, Debug)]
pub struct BugHistoryResponse {
  pub bugs: Vec<BugHistoryContainer>,
}

#[derive(Deserialize, Debug)]
pub struct BugHistoryContainer {
  pub history: Vec<BugHistory>,
  pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct BugHistory {
  pub when: DateTime<Utc>,
  pub who: String,
  pub changes: Vec<BugChange>,
}

#[derive(Deserialize, Debug)]
pub struct BugChange {
  pub added: String,
  pub removed: String,
  pub field_name: String,

  pub attachment_id: Option<u32>,
  pub comment_id: Option<u32>,
  pub comment_count: Option<u32>,
}

pub fn get_user_events(username: String) -> Vec<entity::Event> {
  let mut events: Vec<entity::Event> = Vec::new();
  let mut bug_list_response = reqwest::Client::new().get(&format!("{}/bug?cc={}", API_URL, username)).send().unwrap();
  println!("{}", bug_list_response.status());
  let bug_list: BugListResponse = serde_json::from_str(bug_list_response.text().unwrap().as_str()).unwrap();
  for bug in bug_list.bugs {
    // todo: check if last_change_time exists in stored events already

    // get bug changes
    let bug_history_url = format!("{}/bug/{}/history", API_URL, &bug.id);
    let mut bug_history_response = reqwest::Client::new().get(&bug_history_url).send().unwrap();
    println!("{} ({})", bug_history_response.status(), &bug_history_url);
    let bug_history: BugHistoryResponse = serde_json::from_str(bug_history_response.text().unwrap().as_str()).unwrap();
    let mut history_index = 0;
    for history in bug_history.bugs[0].history.iter().filter(|ref h| h.who.starts_with(username.as_str())) {
      let event = entity::Event{
        id: format!("Bugzilla_{}_{}", &bug.id, &history_index),
        user: format!("{}", &username),
        action: format!("Bugzilla_BugChange"),
        date: history.when,
        title: entity::Element{
          definition: None,
          prefix: None,
          url: Some(format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", &bug.id)),
          text: format!("Bug {}", &bug.id),
          title: None,
          suffix: Some(format!(" {}", &bug.summary)),
        },
        subtitle: None,
        body: Some(entity::Body {
          content: history.changes.iter().map(|ref bug_change| 
            match bug_change.field_name.as_ref() {
              "blocks" | "depends_on" => entity::Element {
                definition: None,
                prefix: Some(format!("{} {} bug ", (match bug_change.removed.as_ref() { "" => "added", _ => "removed" }), (match bug_change.field_name.as_ref() { "blocks" => "blocking of", _ => "dependency on" }))),
                url: Some(format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", &bug_change.added)),
                text: format!("{}", &bug_change.added),
                title: Some(format!("{}", &bug.id)),
                suffix: None,
              },
              /*"cc" | "whiteboard" | "resolution" | "status"*/
              _ => entity::Element {
                definition: None,
                prefix: None,
                url: None,
                text: format!("{} {}: '{}' => '{}'", (match bug_change.removed.as_ref() { "" => "added", _ => match bug_change.added.as_ref() { "" => "removed", _ => "changed" } }), &bug_change.field_name, &bug_change.removed, &bug_change.added),
                title: None,
                suffix: None,
              }
            }
          ).collect(),
          tag: entity::Tag::UnorderedList,
        }),
      };
      println!("{:?}", &event);
      events.push(event);
      history_index += 1;
    }

    // get bug comments
    let bug_comment_url = format!("{}/bug/{}/comment", API_URL, &bug.id);
    let mut bug_comment_response = reqwest::Client::new().get(&bug_comment_url).send().unwrap();
    println!("{} ({})", bug_comment_response.status(), &bug_comment_url);
    let bug_comment_response_body: serde_json::Value = serde_json::from_str(bug_comment_response.text().unwrap().as_str()).unwrap();
    for comment in bug_comment_response_body["bugs"][&bug.id.to_string()]["comments"].as_array().unwrap().iter().filter(|ref c| c["author"].as_str().unwrap().starts_with(username.as_str()))  {
      //println!("{:?}", comment);
      let event = entity::Event{
        id: format!("Bugzilla_{}_c{}", &bug.id, &comment["count"].as_u64().unwrap()),
        user: format!("{}", &username),
        action: format!("Bugzilla_BugComment"),
        date: DateTime::parse_from_rfc3339(&comment["creation_time"].as_str().unwrap()).unwrap().with_timezone(&Utc),
        title: entity::Element{
          definition: None,
          prefix: None,
          url: Some(format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}#c{}", &bug.id, &comment["count"].as_u64().unwrap())),
          text: format!("Bug {} comment {}", &bug.id, &comment["count"].as_u64().unwrap()),
          title: None,
          suffix: Some(format!(" {}", &bug.summary)),
        },
        subtitle: None,
        body: Some(entity::Body{
          content: vec![
            entity::Element{
              definition: None,
              prefix: None,
              url: None,
              text: comment["text"].as_str().unwrap().to_string(),
              title: None,
              suffix: None,
            },
          ],
          tag: entity::Tag::Markdown
        }),
      };
      println!("{:?}", &event);
      events.push(event);
    }
  }
  return events;
}