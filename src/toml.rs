use crate::{
    store::{Store, get_data_dir},
    task::{Task, TaskId},
};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tracing::error;

const TOML_TASKS_EXAMPLE: &'static str = r#"
[[tasks]]
id = "1"
title = "Load tasks from TOML file"
description = "Make sure that project folder is created on initial launch"
status = "Todo"
created_at = "2025-05-06T00:00:00Z"
updated_at = "2025-05-06T00:00:00Z"

[[tasks]]
id = "2"
title = "Session work. Work on Session logging and all that."
status = "Todo"
created_at = "2025-05-06T00:00:00Z"
updated_at = "2025-05-06T00:00:00Z"

[[tasks]]
id = "3" # Example with string TaskId
# id = "a1b2c3d4-e5f6-7890-1234-567890abcdef"
title = "Deploy Initial Version"
status = "Todo"
created_at = "2025-05-06T00:00:00Z"
updated_at = "2025-05-06T00:00:00Z"
"#;

#[derive(Clone, Deserialize)]
pub struct TomlStore {
    tasks: Vec<Task>,
    file_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct TomlStoreFile {
    tasks: Vec<Task>,
}

impl Default for TomlStore {
    fn default() -> Self {
        let mut path = get_data_dir().expect("Failed to get data directory");
        fs::create_dir_all(&path).expect("Failed to create directory");
        path.push("tasks.toml");
        let mut store = Self {
            file_path: path,
            tasks: Vec::new(),
        };
        store.tasks = store.load_all_tasks().unwrap_or_else(|err| {
            error!("Failed to load tasks, starting with example: {}", err);
            if let Ok(example_store) = toml::from_str::<TomlStoreFile>(TOML_TASKS_EXAMPLE) {
                example_store.tasks
            } else {
                Vec::new()
            }
        });
        store
    }
}

impl TomlStore {
    fn ensure_directory_exists(&self) -> Result<()> {
        if let Some(parent_dir) = self.file_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        Ok(())
    }
}

impl Store for TomlStore {
    fn load_all_tasks(&mut self) -> Result<Vec<Task>> {
        if self.file_path.exists() {
            let content = fs::read_to_string(&self.file_path)?;
            let store: TomlStoreFile = toml::from_str(&content)?;
            self.tasks = store.tasks;
            return Ok(self.tasks.clone());
        } else {
            error!(
                "Tasks file not found at {:?}, loading example tasks.",
                self.file_path
            );
            let store: TomlStoreFile = match toml::from_str(TOML_TASKS_EXAMPLE) {
                Ok(tasks_data) => tasks_data,
                Err(err) => {
                    error!("Error parsing example tasks: {}", err);
                    return Err(anyhow!("Failed to parse example tasks: {}", err));
                }
            };
            self.tasks = store.tasks.clone();
            if let Err(e) = self.save_all_tasks(self.tasks.clone()) {
                error!(
                    "Could not save initial example tasks to {:?}: {}",
                    self.file_path, e
                );
            }
            return Ok(self.tasks.clone());
        }
    }

    fn save_all_tasks(&mut self, tasks: Vec<Task>) -> Result<()> {
        self.ensure_directory_exists()?;
        let toml_content = toml::to_string(&TomlStoreFile {
            tasks: tasks.clone(),
        })?;
        fs::write(&self.file_path, toml_content)?;
        self.tasks = tasks;
        Ok(())
    }

    fn append_task(&mut self, task: Task) -> Result<()> {
        let mut current_tasks = self.load_all_tasks()?;
        current_tasks.push(task);
        self.save_all_tasks(current_tasks)
    }

    fn delete_task(&mut self, id: TaskId) -> Result<()> {
        let mut current_tasks = self.load_all_tasks()?;
        current_tasks.retain(|task| task.id != id);
        self.save_all_tasks(current_tasks)
    }

    fn update_task(&mut self, id: TaskId, updated_task: Task) -> Result<()> {
        let mut current_tasks = self.load_all_tasks()?;
        if let Some(task_to_update) = current_tasks.iter_mut().find(|task| task.id == id) {
            *task_to_update = updated_task;
            self.save_all_tasks(current_tasks)
        } else {
            Err(anyhow!("Task with ID {} not found for update", id))
        }
    }
}
