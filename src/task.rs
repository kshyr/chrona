use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumIter};

pub type TaskId = usize;

#[derive(Debug, Default, Clone, EnumIter, AsRefStr, Serialize, Deserialize)]
pub enum TaskStatus {
    #[default]
    Backlog,
    Todo,
    InProgress,
    Done,
}

impl TaskStatus {
    pub fn color(&self) -> Color {
        match self {
            TaskStatus::Backlog => Color::Gray,
            TaskStatus::Todo => Color::Blue,
            TaskStatus::InProgress => Color::Yellow,
            TaskStatus::Done => Color::Green,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::default(),
            description: None,
            status: TaskStatus::Backlog,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
