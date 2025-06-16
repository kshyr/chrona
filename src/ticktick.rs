use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{
    Deserialize, Serialize, Serializer,
    de::{self, IntoDeserializer},
};
use serde_json::Value;
use std::{fs::File, io::Write};
use tracing::{debug, info};
use url::Url;

use crate::{
    store::get_cache_dir,
    task::{TaskId, TaskStatus},
};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    #[serde(rename = "-1")]
    Abandoned = -1,
    #[default]
    #[serde(rename = "0")]
    Normal = 0,
    #[serde(rename = "1")]
    CompletedAlt = 1,
    #[serde(rename = "2")]
    Completed = 2,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TaskPriority {
    #[default]
    #[serde(rename = "0")]
    None,
    #[serde(rename = "1")]
    Low,
    #[serde(rename = "3")]
    Medium,
    #[serde(rename = "5")]
    High,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    TEXT,
    NOTE,
    CHECKLIST,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ViewMode {
    List,
    Kanban,
    Timeline,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SortOptions {
    SortOrder,
    DueDate,
    Tag,
    Priority,
    Project,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
    pub id: String,
    pub title: Option<String>,
    pub status: Option<Status>,
    pub completed_time: Option<String>,
    pub is_all_day: Option<bool>,
    pub sort_order: Option<i64>,
    pub start_date: Option<String>,
    pub time_zone: Option<String>,
    #[serde(default)]
    pub snooze_reminder_time: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskReminder {
    pub id: Option<String>,
    pub trigger: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: TaskId,
    pub project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub etag: String,
    pub status: i64,
    pub priority: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    pub sort_order: i64,
    pub is_floating: bool,
    pub is_all_day: Option<bool>,
    pub kind: Option<Kind>,

    #[serde(serialize_with = "serialize_current_time_as_custom_rfc3339")]
    pub modified_time: DateTime<Utc>,
    #[serde(serialize_with = "serialize_current_time_as_custom_rfc3339")]
    pub created_time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<DateTime<Utc>>,
    #[serde(serialize_with = "serialize_current_time_as_custom_rfc3339_opt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub ex_date: Option<Vec<()>>,
    pub time_zone: String,
    #[serde(rename = "desc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub items: Vec<ChecklistItem>,
    #[serde(default)]
    pub reminders: Vec<Value>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<i64>,
    pub assignee: Option<()>,
}

impl Into<crate::task::Task> for Task {
    fn into(self) -> crate::task::Task {
        crate::task::Task {
            id: self.id,
            title: self.title.unwrap_or("Untitled".into()),
            description: self.description,
            status: if self.status as usize > 0 {
                TaskStatus::Done
            } else {
                TaskStatus::Backlog
            },
            created_at: self.created_time,
            updated_at: self.modified_time,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SortOption {
    pub group_by: SortOptions,
    pub order_by: SortOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTimeline {
    pub range: Option<String>,
    pub sort_type: Option<String>,
    pub sort_option: SortOption,
}

#[derive(Serialize)]
struct TickTickId(TaskId);

impl Default for TickTickId {
    fn default() -> Self {
        let alphabet: [char; 16] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        let id_prefix = String::from("68");
        let id_length = 24 - id_prefix.len();
        let id = id_prefix + &nanoid!(id_length, &alphabet);
        Self(id)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub etag: String,
    pub color: Option<String>,
    pub group_id: Option<String>,
    pub in_all: bool,
    pub kind: Option<String>,
    pub modified_time: String,
    pub sort_option: Option<SortOption>,
    pub permission: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub view_mode: Option<ViewMode>,

    pub timeline: Option<ProjectTimeline>,

    pub closed: Option<bool>,
    pub user_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectGroup {
    pub etag: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub etag: String,
    pub label: String,
    pub name: String,
    pub raw_name: String,
    pub color: Option<String>,
    pub parent: Option<String>,
    pub sort_option: Option<SortOption>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncTaskBean {
    pub update: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchData {
    pub inbox_id: String,
    #[serde(default)]
    pub project_profiles: Vec<Project>,
    #[serde(default)]
    pub project_groups: Option<Vec<ProjectGroup>>,
    pub sync_task_bean: SyncTaskBean,
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub check_point: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserStatus {
    pub username: String,
    pub inbox_id: String,
    pub pro: bool,
    pub pro_end_date: String,
}

fn deserialize_empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: de::Deserializer<'de>,
    T: de::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

pub fn serialize_current_time_as_custom_rfc3339<S>(
    datetime: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    format!("{}", datetime.format("%Y-%m-%dT%H:%M:%S%.3f%z")).serialize(serializer)
}

pub fn serialize_current_time_as_custom_rfc3339_opt<S>(
    datetime: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    Ok(format!("{}", datetime.unwrap().format("%Y-%m-%dT%H:%M:%S%.3f%z")).serialize(serializer)?)
}

const X_DEVICE_HEADER: &str = r#"{"platform":"web","os":"macOS 10.15.7","device":"Chrome 121.0.0.0","name":"","version":6310,"id":"68386f4eac80454b13a089b2","channel":"website","campaign":"","websocket":""}"#;
const USER_AGENT_HEADER: &str = r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"#;

#[derive(Serialize)]
struct LoginBody {
    username: String,
    password: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct AddTaskPayload {
    add: Vec<Task>,
    add_attachments: Vec<()>,
    delete: Vec<()>,
    delete_attachments: Vec<()>,
    update: Vec<()>,
    update_attachments: Vec<()>,
}

fn cache_json(path: &str, json: String) -> Result<()> {
    let mut file_path = get_cache_dir()?;
    file_path.push(path);
    if !file_path.set_extension("json") {
        return Err(anyhow!("Failed to set cache file extension"));
    }
    let mut json_file = File::create(file_path)?;
    json_file.write_all(json.as_bytes())?;
    Ok(())
}

fn body_to_pretty_json(body: ureq::Body) -> Result<String> {
    let parsed_json: Value = serde_json::from_reader(body.into_reader())?;
    let pretty_string = serde_json::to_string_pretty(&parsed_json)?;
    Ok(pretty_string)
}

#[derive(Debug)]
pub struct TickTickClient {
    http: ureq::Agent,
    base_url: Url,
    cookies: Vec<String>,
    state: ClientState,
}

#[derive(Debug, Default)]
struct ClientState {
    inbox_id: Option<String>,
    projects: Option<Vec<Project>>,
    project_groups: Option<Vec<ProjectGroup>>,
    tags: Option<Vec<Tag>>,
    tasks: Option<Vec<Task>>,
}

impl TickTickClient {
    pub fn new(username: String, password: String) -> Result<Self> {
        let agent = ureq::agent();

        // let mut session_json_path = get_cache_dir()?;
        // session_json_path.push("session.json");
        // let f = File::open(session_json_path)?;
        // let buf = BufReader::new(f);
        // let session: Option<Session> = serde_json::from_reader(buf).ok();
        // debug!("Session: {session:?}");

        let mut client = Self {
            http: agent,
            base_url: Url::parse("https://api.ticktick.com/api/v2/")?,
            cookies: vec![],
            state: ClientState::default(),
        };

        client.login(username, password)?;
        info!("Login successful, session cookies stored in agent.");

        client.sync_data()?;
        info!("Initial data sync complete.");

        debug!("{:?}", client.state);

        let res = client.create_task("Hi from Chrona".into(), None, None, None);
        debug!("CEREATEAASK: {:?}", res);

        Ok(client)
    }

    pub fn inbox_id(&self) -> Option<String> {
        self.state.inbox_id.clone()
    }

    pub fn get_projects(&self) -> Option<Vec<Project>> {
        self.state.projects.clone()
    }

    pub fn get_tasks(&self) -> Option<Vec<Task>> {
        self.state.tasks.clone()
    }

    fn login(&mut self, username: String, password: String) -> Result<()> {
        let login_url = self.endpoint("user/signon?wc=true&remember=true")?;
        let body = LoginBody { username, password };
        info!("Attempting login to {}", login_url);

        let response = self
            .http
            .post(login_url.as_str())
            .header("User-Agent", USER_AGENT_HEADER)
            .header("x-device", X_DEVICE_HEADER)
            .header("content-type", "application/json")
            .send(&serde_json::to_string(&body)?)?;

        let mut cookie_kv: Vec<(&str, &str)> = vec![];
        response
            .headers()
            .get_all("set-cookie")
            .iter()
            .for_each(|sc| {
                sc.to_str().unwrap().split("; ").for_each(|v| {
                    debug!("{v}");
                    let kv: Vec<&str> = v.trim().split("=").collect();
                    debug!("{kv:?}");
                    let mut exists = false;
                    cookie_kv.iter().for_each(|c| {
                        if c.1 == "\"\"" {
                            return;
                        }

                        if c.0 == kv[0] {
                            exists = true;
                        }
                    });
                    if !exists {
                        cookie_kv.push((kv[0], kv.get(1).unwrap_or(&"")));
                    }
                });
            });

        let cookie_values: Vec<String> = cookie_kv
            .iter()
            .map(|(k, v)| {
                if v != &"" {
                    k.to_string() + "=" + v
                } else {
                    k.to_string()
                }
            })
            .collect();

        self.cookies = cookie_values;
        let cookie_header = self.cookies.join("; ") + ";";
        debug!("Cookie header: {:?}", cookie_header);

        Ok(())
    }

    fn sync_data(&mut self) -> Result<()> {
        let sync_url = self.endpoint("batch/check/0")?;
        info!("Fetching initial data from {}", sync_url);

        let response = self
            .http
            .get(sync_url.as_str())
            .header("x-device", X_DEVICE_HEADER)
            .header("User-Agent", USER_AGENT_HEADER)
            .header("Cookie", &(self.cookies.join(";") + ";"))
            .call()
            .context("Failed to call sync endpoint")?;

        if response.status() == 200 {
            let json = body_to_pretty_json(response.into_body())?;
            debug!("SyncDataResponse: {json:?}");

            let sync_data: BatchData = serde_json::from_str(&json)?;

            cache_json("batch_data", json)?;
            info!("Successfully deserialized sync data.");

            self.state.inbox_id = Some(sync_data.inbox_id);
            self.state.projects = Some(sync_data.project_profiles);
            self.state.project_groups = sync_data.project_groups;
            self.state.tags = Some(sync_data.tags);
            self.state.tasks = Some(sync_data.sync_task_bean.update);

            Ok(())
        } else {
            Err(anyhow!(
                "Failed to sync data. Status: {}. Body: {:?}",
                response.status(),
                response.body()
            ))
        }
    }

    pub fn create_task(
        &self,
        title: String,
        project_id: Option<String>,
        content: Option<String>,
        priority: Option<TaskPriority>,
    ) -> Result<()> {
        let url = self.endpoint("batch/task")?;
        let project_id = project_id.unwrap_or_else(|| self.inbox_id().unwrap());
        let now = chrono::Utc::now();

        let new_task = Task {
            id: TickTickId::default().0,
            title: Some(title),
            is_all_day: Some(true),
            content: Some(content.unwrap_or_default()),
            progress: Some(0),
            priority: priority.unwrap_or_default() as i64,
            ex_date: Some(vec![]),
            start_date: Some(now),
            sort_order: -(now.timestamp_millis() * 4096),
            created_time: now,
            modified_time: now,
            project_id,
            time_zone: "America/New_York".into(),
            ..Default::default()
        };

        debug!("New Task:\n{}", serde_json::to_string(&new_task)?);

        let payload = AddTaskPayload {
            add: vec![new_task],
            ..Default::default()
        };

        let payload_string = serde_json::to_string_pretty(&payload)?;

        info!("Creating task with payload: {}", payload_string);

        let response = self
            .http
            .post(url.as_str())
            .header("x-device", X_DEVICE_HEADER)
            .header("User-Agent", USER_AGENT_HEADER)
            .header("Cookie", self.cookies.join(";") + ";")
            .header("content-type", "application/json")
            .send(&payload_string)
            .context("Failed to send create task request")?;

        if response.status() == 200 {
            info!("Successfully sent create task request.");
            Ok(())
        } else {
            Err(anyhow!(
                "Failed to create task. Status: {}. Body: {:?}",
                response.status(),
                response.body()
            ))
        }
    }

    fn endpoint(&self, path: &str) -> Result<Url> {
        self.base_url
            .join(path)
            .context(format!("Failed to join URL path: {}", path))
    }
}
