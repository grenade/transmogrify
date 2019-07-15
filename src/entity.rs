use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {

  pub id: String,

  pub action: String,

  pub date: DateTime<Utc>,

  pub title: Element,

  #[serde(skip_serializing_if="Option::is_none")]
  pub subtitle: Option<Element>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub body: Option<Body>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
  pub content: Vec<Element>,
  pub tag: Tag,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Element {

  #[serde(skip_serializing_if="Option::is_none")]
  pub definition: Option<KeyValuePair>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub prefix: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub url: Option<String>,

  pub text: String,

  #[serde(skip_serializing_if="Option::is_none")]
  pub title: Option<String>,

  #[serde(skip_serializing_if="Option::is_none")]
  pub suffix: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Tag {
  DataList,
  DataTerm,
  Icon,
  OrderedList,
  Paragraph,
  Paragraphs,
  UnorderedList,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Activity {
  GithubPush,
  BugzillaEvent,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyValuePair {
  pub name: Tag,

  #[serde(skip_serializing_if="Option::is_none")]
  pub value: Option<String>,
}