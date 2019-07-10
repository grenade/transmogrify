use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct Event {
  pub id: String,
  pub date: DateTime<Utc>,
  pub title: Element,
  pub subtitle: Element,
  pub body: Body,
}

#[derive(Deserialize, Debug)]
pub struct Body {
  pub content: Vec<Element>,
  pub tag: Tag,
}

#[derive(Deserialize, Debug)]
pub struct Element {
  pub definition: Option<KeyValuePair>,
  pub prefix: Option<String>,
  pub url: Option<String>,
  pub text: String,
  pub title: Option<String>,
  pub suffix: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum Tag {
  DataList,
  DataTerm,
  Icon,
  OrderedList,
  Paragraph,
  Paragraphs,
  UnorderedList,
}

#[derive(Deserialize, Debug)]
pub enum Activity {
  GithubPush,
  BugzillaEvent,
}

#[derive(Deserialize, Debug)]
pub struct KeyValuePair {
  pub name: Tag,
  pub value: Option<String>,
}