use std::{fs::File, io::Write, str::FromStr};

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use oauth2::{ureq, url::Url};
use serde::{Deserialize, Serialize, de, de::IntoDeserializer};
use serde_json::Value;
use tracing::{debug, info};

use crate::task::TaskStatus;

/// The completion status of a checklist item or task.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Task is abandoned or "won't do".
    #[serde(rename = "-1")]
    Abandoned = -1,
    /// Task is active/normal.
    #[serde(rename = "0")]
    Normal = 0,
    /// An alternative representation for completed status.
    #[serde(rename = "1")]
    CompletedAlt = 1,
    /// Task is completed.
    #[serde(rename = "2")]
    Completed = 2,
}

/// Priority of a task.
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

/// Defines the kind of a task, which can be a standard task, a note, or a checklist.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    TEXT,
    NOTE,
    CHECKLIST,
}

/// Defines the view mode for a project.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ViewMode {
    List,
    Kanban,
    Timeline,
}

/// Defines how tasks are sorted or grouped.
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

/// Represents a single item in a checklist task.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
    /// ID of the checklist item.
    pub id: String,
    /// Title of the checklist item.
    pub title: Option<String>,
    /// The completion status of the checklist item.
    pub status: Option<Status>,
    /// Completed time in `yyyy-MM-dd'T'HH:mm:ssZ` format.
    pub completed_time: Option<String>,
    /// The task is due any time on the due date, rather than at a specific time.
    pub is_all_day: Option<bool>,
    /// The order of the checklist item.
    pub sort_order: Option<i64>,
    /// Start date and time in `yyyy-MM-dd'T'HH:mm:ssZ` format.
    pub start_date: Option<String>,
    /// IANA time zone. Example: 'America/Los_Angeles'.
    pub time_zone: Option<String>,
    #[serde(default)]
    pub snooze_reminder_time: serde_json::Value,
}

/// Represents a reminder for a task.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskReminder {
    /// Reminder ID.
    pub id: Option<String>,
    /// Reminder trigger (iCal format).
    pub trigger: String,
}

/// Represents a TickTick Task.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub etag: String,
    pub status: i64,   // Deserialize as integer
    pub priority: i64, // Deserialize as integer
    pub sort_order: i64,
    pub is_floating: bool,
    pub kind: Option<String>, // Deserialize as String

    // Make all other potentially missing fields optional
    pub modified_time: String,
    pub created_time: Option<String>,
    pub completed_time: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub time_zone: Option<String>,
    #[serde(rename = "desc")]
    pub description: Option<String>,
    #[serde(default)]
    pub items: Vec<ChecklistItem>,
    #[serde(default)]
    pub reminders: Vec<Value>, // Use Value for flexibility
    #[serde(default)]
    pub tags: Vec<String>,
    pub parent_id: Option<String>,
    #[serde(default)]
    pub child_ids: Vec<String>,
    pub creator: Option<i64>,
}

impl Into<crate::task::Task> for Task {
    fn into(self) -> crate::task::Task {
        crate::task::Task {
            id: self.id,
            title: self.title.unwrap_or("Untitled".into()),
            description: self.description,
            status: if self.status > 0 {
                TaskStatus::Done
            } else {
                TaskStatus::Backlog
            },
            created_at: chrono::DateTime::from_str(&self.created_time.unwrap())
                .ok()
                .unwrap(),
            updated_at: chrono::DateTime::from_str(&self.modified_time)
                .ok()
                .unwrap(),
        }
    }
}

/// Represents how tasks are sorted within a project or tag.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SortOption {
    /// How tasks are grouped.
    pub group_by: SortOptions,
    /// How tasks are ordered.
    pub order_by: SortOptions,
}

/// An unknown timeline-related model.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTimeline {
    pub range: Option<String>,
    pub sort_type: Option<String>,
    pub sort_option: SortOption,
}

/// Represents a TickTick Project (also known as a List).
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
    pub modified_time: String, // Keep as String
    pub sort_option: Option<SortOption>,
    pub permission: Option<String>,

    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub view_mode: Option<ViewMode>,

    // Use Option for nested objects that can be null
    pub timeline: Option<ProjectTimeline>,

    // Add other fields from the JSON
    pub closed: Option<bool>,
    pub user_count: Option<i64>,
}

/// Represents a Project Group (a folder for projects).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectGroup {
    /// ETag of the project group object.
    pub etag: String,
    /// ID of the project group.
    pub id: String,
    /// Name of the project group.
    pub name: String,
}

/// Represents a TickTick Tag.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// ETag of the tag object.
    pub etag: String,
    /// Name of the tag, as it appears in the UI.
    pub label: String,
    /// Name of the tag, similar to the label but lowercase.
    pub name: String,
    /// Original name of the tag, used to identify it.
    pub raw_name: String,
    /// Color of the tag, eg. '#F18181'.
    pub color: Option<String>,
    /// Name of the parent tag, if nested.
    pub parent: Option<String>,
    /// How to sort the tasks within the tag.
    pub sort_option: Option<SortOption>,
}

/// Represents the `syncTaskBean` portion of the batch response.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncTaskBean {
    /// List of all active tasks for the user.
    pub update: Vec<Task>,
}

/// The response from the `/api/v2/batch/check/0` endpoint.
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

/// Represents the user's subscription status.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserStatus {
    pub username: String,
    pub inbox_id: String,
    pub pro: bool,
    pub pro_end_date: String,
}

/// Helper to deserialize fields that might be an empty string into `None`.
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

// Static headers for API requests.
const X_DEVICE_HEADER: &str = r#"{"platform":"web","os":"macOS 10.15.7","device":"Chrome 121.0.0.0","name":"","version":6310,"id":"68386f4eac80454b13a089b2","channel":"website","campaign":"","websocket":""}"#;
const USER_AGENT_HEADER: &str = r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"#;

#[derive(Serialize)]
struct LoginBody {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AddTaskPayload {
    add: Vec<CreateTask>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CreateTask {
    project_id: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<TaskPriority>,
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
        println!("{:?}", client.state);

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
            .set("User-Agent", USER_AGENT_HEADER)
            .set("x-device", X_DEVICE_HEADER)
            .set("content-type", "application/json")
            .send_string(&serde_json::to_string(&body)?)
            .unwrap();

        let cookies = response.all("set-cookie").join("; ") + ";";
        self.cookies = cookies.split(';').map(|s| s.trim().to_string()).collect();
        debug!("Cookies: {:?}", self.cookies);

        Ok(())
    }

    fn sync_data(&mut self) -> Result<()> {
        let sync_url = self.endpoint("batch/check/0")?;
        info!("Fetching initial data from {}", sync_url);

        let response = self
            .http
            .get(sync_url.as_str())
            .set("x-device", X_DEVICE_HEADER)
            .set("User-Agent", USER_AGENT_HEADER)
            .set("Cookie", &(self.cookies.join(";") + ";"))
            .call()
            .context("Failed to call sync endpoint")?;

        if response.status() == 200 {
            let json_string = response.into_string()?;

            let pretty_json_string =
                if let Ok(parsed_json) = serde_json::from_str::<Value>(&json_string) {
                    serde_json::to_string_pretty(&parsed_json).unwrap()
                } else {
                    return Err(anyhow!("Could not even parse as a raw JSON Value."));
                };
            debug!("SyncDataResponse: {pretty_json_string:?}");
            let mut json_file = File::create(".cache/batch_data.json")?;
            json_file.write_all(pretty_json_string.as_bytes())?;

            let sync_data: BatchData = serde_json::from_str(&pretty_json_string)?;

            info!("Successfully deserialized sync data.");

            self.state.inbox_id = Some(sync_data.inbox_id);
            self.state.projects = Some(sync_data.project_profiles);
            self.state.project_groups = sync_data.project_groups;
            self.state.tags = Some(sync_data.tags);
            self.state.tasks = Some(sync_data.sync_task_bean.update);

            Ok(())
        } else {
            Err(anyhow!(
                "Failed to sync data. Status: {}. Body: {}",
                response.status(),
                response.into_string().unwrap_or_default()
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

        let new_task = CreateTask {
            title,
            project_id,
            content,
            priority,
        };

        let payload = AddTaskPayload {
            add: vec![new_task],
        };
        let payload_string = serde_json::to_string(&payload)?;

        info!("Creating task with payload: {}", payload_string);

        let response = self
            .http
            .post(url.as_str())
            .set("x-device", X_DEVICE_HEADER)
            .set("User-Agent", USER_AGENT_HEADER)
            .send_string(&payload_string)
            .context("Failed to send create task request")?;

        if response.status() == 200 {
            info!("Successfully sent create task request.");
            Ok(())
        } else {
            Err(anyhow!(
                "Failed to create task. Status: {}. Body: {}",
                response.status(),
                response.into_string().unwrap_or_default()
            ))
        }
    }

    fn endpoint(&self, path: &str) -> Result<Url> {
        self.base_url
            .join(path)
            .context(format!("Failed to join URL path: {}", path))
    }
}
