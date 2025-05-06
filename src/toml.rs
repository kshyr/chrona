use serde::Deserialize;

use crate::{
    App,
    task::{Task, TaskId},
};

pub trait Store {
    fn new() -> Self;
    fn save_task(&mut self, task: Task);
    fn delete_task(&mut self, id: TaskId);
    fn get_task(&self, id: TaskId) -> Option<&Task>;
    fn get_tasks(&self) -> &Vec<Task>;
}

#[derive(Debug, Clone, Deserialize)]
struct TasksContainer {
    tasks: Vec<Task>,
}

fn load_tasks() -> Vec<Task> {
    let container: TasksContainer = match toml::from_str(
        r#"
    [[tasks]]
    id = 1
    title = "Implement User Authentication"
    description = "Set up user registration, login, and session management."
    status = "InProgress"
    created_at = "1523-08-20T23:54:33.000011235Z"
    updated_at = "1523-08-20T23:54:33.000011235Z"

    [[tasks]]
    id = 2
    title = "Design Database Schema"
    description = "Define tables, columns, relationships, and indexes for the project."
    status = "Todo"
    created_at = "2025-04-28T09:00:00Z"
    updated_at = "2025-04-28T09:00:00Z"

    [[tasks]]
    id = 3 # Example with string TaskId
    # id = "a1b2c3d4-e5f6-7890-1234-567890abcdef"
    title = "Deploy Initial Version"
    description = "Deploy the application to the staging server."
    status = "Done"
    created_at = "2025-04-15T14:00:00Z"
    updated_at = "2025-04-30T11:45:10Z"
    "#,
    ) {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("Error loading tasks: {}", err);
            TasksContainer { tasks: Vec::new() }
        }
    };
    container.tasks
}

#[derive(Debug, Clone, Deserialize)]
pub struct TomlStore {
    pub tasks: Vec<Task>,
}

impl Store for TomlStore {
    fn new() -> Self {
        Self {
            tasks: load_tasks(),
        }
    }

    fn save_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn delete_task(&mut self, id: TaskId) {
        self.tasks.retain(|task| task.id != id);
    }

    fn get_task(&self, id: TaskId) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

impl From<App> for TomlStore {
    fn from(app: App) -> Self {
        TomlStore {
            tasks: app.task_list1.tasks,
        }
    }
}
