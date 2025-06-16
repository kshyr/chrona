use crate::task::{Task, TaskId};
use anyhow::{Result, anyhow};
use directories::BaseDirs;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub trait Store {
    fn load_all_tasks(&mut self) -> Result<Vec<Task>>;
    fn save_all_tasks(&mut self, tasks: Vec<Task>) -> Result<()>;

    fn append_task(&mut self, task: Task) -> Result<()>;
    fn delete_task(&mut self, id: TaskId) -> Result<()>;
    fn update_task(&mut self, id: TaskId, task: Task) -> Result<()>;
}

fn get_dir(path: &Path, env_key: &'static str) -> Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var(env_key) {
        PathBuf::from(s)
    } else if path.exists() {
        let mut dir = path.to_path_buf();
        dir.push("chrona");
        dir
    } else {
        return Err(anyhow!(
            "Unable to find config directory for ratatui-template"
        ));
    };

    fs::create_dir_all(&directory)?;
    Ok(directory)
}

pub fn get_data_dir() -> Result<PathBuf> {
    get_dir(BaseDirs::new().unwrap().data_dir(), "CHRONA_DATA_DIR")
}

pub fn get_cache_dir() -> Result<PathBuf> {
    get_dir(BaseDirs::new().unwrap().cache_dir(), "CHRONA_CACHE_DIR")
}

pub fn get_state_dir() -> Result<PathBuf> {
    let base_dirs = BaseDirs::new().unwrap();
    get_dir(
        base_dirs.state_dir().unwrap_or(base_dirs.data_local_dir()),
        "CHRONA_STATE_DIR",
    )
}

// pub fn get_config_dir() -> Result<PathBuf> {
//     get_dir(BaseDirs::new().unwrap().config_dir(), "CHRONA_CONFIG_DIR")
// }
