pub type TaskId = u32;

#[derive(Debug, Default)]
enum TaskStatus {
    #[default]
    Backlog,
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Default)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            id: 0,
            title: title.to_string(),
            description: description.to_string(),
            status: TaskStatus::Backlog,
        }
    }
}
