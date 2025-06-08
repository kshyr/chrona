use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use oauth2::{AccessToken, ureq, url::Url};
use serde::{Deserialize, Serialize, de};
use tracing::debug;

#[repr(i32)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ChecklistItemStatus {
    Normal = 0,
    Completed = 1,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
    pub id: String,
    pub title: String,
    pub status: ChecklistItemStatus,
    pub completed_time: Option<DateTime<Utc>>,
    pub is_all_day: bool,
    pub sort_order: i64,
    pub start_date: Option<DateTime<Utc>>,
    pub time_zone: String,
}

#[repr(i32)]
#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Normal = 0,
    Completed = 2,
}

#[repr(i32)]
#[derive(Serialize, Deserialize, Debug)]
pub enum TaskPriority {
    None = 0,
    Low = 1,
    Medium = 3,
    High = 5,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub is_all_day: bool,
    pub completed_time: Option<DateTime<Utc>>,
    pub content: Option<String>,
    pub desc: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub items: Vec<ChecklistItem>,
    pub priority: TaskPriority,
    #[serde(default)]
    pub reminders: Vec<String>,
    pub repeat_flag: Option<String>,
    pub sort_order: i64,
    pub start_date: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ViewMode {
    Empty,
    List,
    Kanban,
    Timeline,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Permission {
    Read,
    Write,
    Comment,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Kind {
    Task,
    Note,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub sort_order: i64,
    pub closed: bool,
    pub group_id: Option<String>,
    #[serde(deserialize_with = "deserialize_viewmode")]
    pub view_mode: Option<ViewMode>,
    pub permission: Permission,
    pub kind: Kind,
}

fn deserialize_viewmode<'de, D>(deserializer: D) -> Result<Option<ViewMode>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.is_empty() {
        Ok(None)
    } else {
        ViewMode::deserialize(de::IntoDeserializer::into_deserializer(s.as_str())).map(Some)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    id: String,
    project_id: String,
    name: String,
    sort_order: i64,
}

#[derive(Debug)]
pub struct TickTickClient {
    http: ureq::Agent,
    base_url: Url,
    access_token: AccessToken,
}

impl TickTickClient {
    pub fn new(access_token: AccessToken) -> Result<Self> {
        Ok(Self {
            http: ureq::agent(),
            base_url: Url::parse("https://api.ticktick.com/open/v1/")?,
            access_token,
        })
    }

    fn get<T: de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self
            .http
            .get(self.endpoint(path).as_str())
            .set(
                "Authorization",
                &format!("Bearer {}", self.access_token.secret()),
            )
            .call()?;

        let json_string = response.into_string()?;
        debug!("GET {}: {}", path, json_string);
        serde_json::from_str(&json_string)
            .map_err(|e| anyhow!("Failed to deserialize response from {}: {}", path, e))
    }

    pub fn get_projects(&self) -> Result<Vec<Project>> {
        self.get("project")
    }

    fn endpoint(&self, path: &str) -> Url {
        self.base_url.join(path).expect("Failed to join URL")
    }
}
