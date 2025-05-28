use crate::task::{Task, TaskId};
use color_eyre::eyre;
use directories::BaseDirs;
use std::{fs, path::PathBuf};

pub trait Store {
    fn load_all_tasks(&mut self) -> color_eyre::Result<Vec<Task>>;
    fn save_all_tasks(&mut self, tasks: Vec<Task>) -> color_eyre::Result<()>;

    fn append_task(&mut self, task: Task) -> color_eyre::Result<()>;
    fn delete_task(&mut self, id: TaskId) -> color_eyre::Result<()>;
    fn update_task(&mut self, id: TaskId, task: Task) -> color_eyre::Result<()>;
}

pub fn get_data_dir() -> color_eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("CHRONA_DATA_DIR") {
        PathBuf::from(s)
    } else if let Some(base_dirs) = BaseDirs::new() {
        let mut data_dir = base_dirs.data_dir().to_path_buf();
        data_dir.push("chrona");
        data_dir
    } else {
        return Err(eyre::eyre!(
            "Unable to find data directory for ratatui-template"
        ));
    };
    fs::create_dir_all(&directory)?;
    Ok(directory)
}

pub fn get_config_dir() -> color_eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("CHRONA_CONFIG_DIR") {
        PathBuf::from(s)
    } else if let Some(base_dirs) = BaseDirs::new() {
        let mut config_dir = base_dirs.config_dir().to_path_buf();
        config_dir.push("chrona");
        config_dir
    } else {
        return Err(eyre::eyre!(
            "Unable to find config directory for ratatui-template"
        ));
    };
    fs::create_dir_all(&directory)?;
    Ok(directory)
}
