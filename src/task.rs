use serde::Deserialize;

pub type TaskId = usize;

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub enum TaskStatus {
    #[default]
    Backlog,
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Task {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            id: 0,
            title: title.to_string(),
            description: description.to_string(),
            status: TaskStatus::Backlog,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
