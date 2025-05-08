use std::{error, fs, path::PathBuf};

use color_eyre::eyre;
use serde::{Deserialize, Serialize};

use crate::{
    App,
    store::{Store, get_config_dir},
    task::{Task, TaskId},
};

const TOML_TASKS_EXAMPLE: &'static str = r#"
[[tasks]]
id = 1
title = "Load tasks from TOML file"
description = "Make sure that project folder is created on initial launch"
status = "Todo"
created_at = "2025-05-06T00:00:00Z"
updated_at = "2025-05-06T00:00:00Z"

[[tasks]]
id = 2
title = "Session work. Work on Session logging and all that."
status = "Todo"
created_at = "2025-05-06T00:00:00Z"
updated_at = "2025-05-06T00:00:00Z"

[[tasks]]
id = 3 # Example with string TaskId
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

impl TomlStore {
    pub fn new() -> color_eyre::Result<Self> {
        let mut path = get_config_dir()?;
        // Ensure the directory exists
        fs::create_dir_all(&path)?;
        path.push("tasks.toml");
        let mut store = Self {
            file_path: path,
            tasks: Vec::new(),
        };
        store.tasks = store.load_all_tasks().unwrap();
        Ok(store)
    }

    /// Ensures the directory for the tasks file exists.
    fn ensure_directory_exists(&self) -> color_eyre::Result<()> {
        if let Some(parent_dir) = self.file_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        Ok(())
    }
}

impl Store for TomlStore {
    fn load_all_tasks(&mut self) -> color_eyre::Result<Vec<Task>> {
        if !self.tasks.is_empty() {
            return Ok(self.tasks.clone());
        }

        let store: TomlStoreFile = match toml::from_str(TOML_TASKS_EXAMPLE) {
            Ok(tasks) => tasks,
            Err(err) => {
                eprintln!("Error loading tasks: {}", err);
                TomlStoreFile { tasks: Vec::new() }
            }
        };
        Ok(store.tasks)
    }

    fn save_all_tasks(&mut self, tasks: Vec<Task>) -> color_eyre::Result<()> {
        self.ensure_directory_exists()?;
        let toml_content = toml::to_string(&TomlStoreFile { tasks })?;
        fs::write(&self.file_path, toml_content)?;
        Ok(())
    }

    fn append_task(&mut self, task: Task) -> color_eyre::Result<()> {
        self.tasks.push(task);
        self.save_all_tasks(self.tasks.clone())
    }

    fn delete_task(&mut self, id: TaskId) -> color_eyre::Result<()> {
        self.tasks.retain(|task| task.id != id);
        self.save_all_tasks(self.tasks.clone())
    }

    fn update_task(&mut self, id: TaskId, updated_task: Task) -> color_eyre::Result<()> {
        for task in self.tasks.iter_mut() {
            if task.id == id {
                *task = updated_task.clone();
                return Ok(());
            }
        }
        Err(eyre::eyre!("Task with ID {} not found for update", id))
    }
}
