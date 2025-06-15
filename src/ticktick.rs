use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use oauth2::ureq;
use oauth2::url::Url;
use serde::{
    Deserialize, Serialize,
    de::{self, IntoDeserializer},
};
use tracing::{debug, info};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ChecklistItemStatus {
    #[serde(rename = "0")]
    Normal,
    #[serde(rename = "1")]
    Completed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    #[serde(rename = "0")]
    Normal,
    #[serde(rename = "2")]
    Completed,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub sort_order: i64,
    pub title: String,
    #[serde(rename = "desc")]
    pub description: String,
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub is_all_day: bool,
    pub completed_time: Option<DateTime<Utc>>,
    pub content: Option<String>,
    #[serde(default)]
    pub items: Vec<ChecklistItem>,
    pub priority: TaskPriority,
    #[serde(default)]
    pub reminders: Vec<String>,
    pub status: TaskStatus,
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ViewMode {
    List,
    Kanban,
    Timeline,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub sort_order: i64,
    pub closed: bool,
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub view_mode: Option<ViewMode>,
    pub kind: String, // e.g., "TASK" or "NOTE"
}

/// The response from the `/api/v2/batch/check` endpoint contains all the initial data
/// needed to populate the app, including projects, tasks, user info, etc.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SyncData {
    pub inbox_id: String,
    pub projects: Vec<Project>,
    pub project_profiles: Vec<Project>,
    #[serde(default)]
    pub task_folders: Vec<serde_json::Value>,
    #[serde(default)]
    pub tasks: Vec<Task>,
    pub user_profile: UserProfile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub email: String,
    pub inbox_id: String,
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

#[derive(Serialize)]

struct XDevice {
    platform: &'static str,
    os: &'static str,
    name: &'static str,
    device: &'static str,
    version: u16,
    id: String,
    channel: &'static str,
    campaign: &'static str,
    websocket: &'static str,
}

impl Default for XDevice {
    fn default() -> Self {
        let alphabet: [char; 16] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        let version = 6240;
        let id_prefix = String::from("66");
        let id_length = 24 - id_prefix.len();
        let id = id_prefix + &nanoid!(id_length, &alphabet);

        Self {
            id,
            version,
            platform: "web",
            os: "Windows 10",
            device: "Firefox 117.0",
            channel: "website",
            name: "",
            campaign: "",
            websocket: "",
        }
    }
}

const X_DEVICE_HEADER: &str = r#"{"platform":"web","os":"macOS 10.15.7","device":"Chrome 121.0.0.0","name":"","version":6310,"id":"68386f4eac80454b13a089b2","channel":"website","campaign":"","websocket":""}"#;
const USER_AGENT_HEADER: &str = r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"#;

#[derive(Serialize)]
struct LoginBody {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct CreateTaskBody {}

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
}

impl TickTickClient {
    pub fn new(username: String, password: String) -> Result<Self> {
        let agent = ureq::agent();

        let mut client = Self {
            http: agent,
            base_url: Url::parse("https://api.ticktick.com/api/v2/")?,
            state: ClientState::default(),
            cookies: vec![],
        };

        client.login(username, password)?;
        info!("Login successful, session cookies stored in agent.");

        // client.sync_data()?;
        // info!("Initial data sync complete.");

        debug!(
            "{:?}",
            client.create_task("Hi from Chrona".into(), None, None, None)
        );

        Ok(client)
    }

    pub fn inbox_id(&self) -> Option<String> {
        self.state.inbox_id.clone()
    }

    pub fn get_projects(&self) -> Option<Vec<Project>> {
        self.state.projects.clone()
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
            .set("content-type", "application/json")
            .set("Cookie", &(self.cookies.join(";") + ";"))
            .call()
            .context("Failed to call sync endpoint")?;

        if response.status() == 200 {
            let json_string = response.into_string()?;
            debug!("SyncDataResponse: {json_string:?}");
            let sync_data: SyncData = serde_json::from_str(&json_string)?;

            info!("Successfully deserialized sync data.");

            self.state.inbox_id = Some(sync_data.inbox_id);
            self.state.projects = Some(sync_data.projects);

            Ok(())
        } else {
            let status = response.status();
            let body = response.into_string().unwrap_or_default();
            Err(anyhow!(
                "Failed to sync data. Status: {}. Body: {}",
                status,
                body
            ))
        }
    }

    pub fn create_task(
        &self,
        title: String,
        project_id: Option<String>,
        content: Option<String>,
        priority: Option<TaskPriority>,
    ) -> Result<Task> {
        let url = self.endpoint("task")?;
        let project_id = project_id.unwrap_or_else(|| self.inbox_id().unwrap());

        let payload = serde_json::json!({
            "title": title,
            "projectId": project_id,
            "content": content,
            "priority": priority.unwrap_or_default(),
            // Other fields like dueDate, reminders, etc., can be added here.
            // "timeZone": "America/New_York", // Usually required if dates are involved
        })
        .to_string();

        info!("Creating task with payload: {}", payload);

        let response = self
            .http
            .post(url.as_str())
            .send_string(&payload)
            .context("Failed to send create task request")?;

        if response.status() == 200 || response.status() == 201 {
            let created_task: Task = serde_json::from_str(&response.into_string()?)?;
            info!("Successfully created task: {}", created_task.id);
            Ok(created_task)
        } else {
            let status = response.status();
            let body = response.into_string().unwrap_or_default();
            Err(anyhow!(
                "Failed to create task. Status: {}. Body: {}",
                status,
                body
            ))
        }
    }

    fn endpoint(&self, path: &str) -> Result<Url> {
        self.base_url
            .join(path)
            .context(format!("Failed to join URL path: {}", path))
    }
}
