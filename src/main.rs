mod app;
mod help;
mod page;
mod pages;
mod store;
mod task;
mod ticktick;
mod toml;
mod ui;
mod widgets;

use anyhow::{Result, anyhow};
use app::App;
use dotenv::dotenv;
use std::fs::{File, create_dir, read_dir};
use tracing::info;
use tracing_subscriber::{fmt::layer, layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::store::get_state_dir;

fn log_file_timestamp() -> String {
    chrono::Local::now().format("%Y%m%d-%H%M").to_string()
}

fn init_log_file() -> Result<File> {
    let mut path = get_state_dir()?;
    path.push("logs");

    let logs_dir = path.clone();

    if !path.exists() {
        create_dir(path.clone())?;
    }

    let timestamp = log_file_timestamp();
    path.push(timestamp.clone() + ".log");

    // TODO: refactor
    if path.exists() {
        path.pop();
        let mut count = 0;
        for entry in read_dir(logs_dir)? {
            if let Ok(file_name) = entry?.file_name().into_string() {
                if file_name.contains(&timestamp) {
                    count += 1;
                }
            } else {
                return Err(anyhow!("Failed to convert file name to String"));
            }
        }
        path.push(log_file_timestamp() + "(" + &count.to_string() + ").log")
    }

    let log_file = File::create(path)?;
    Ok(log_file)
}

fn main() -> Result<()> {
    dotenv().ok();

    let log_file = init_log_file()?;
    tracing_subscriber::registry()
        .with(layer().with_writer(log_file))
        .init();

    let terminal = ratatui::init();
    let result = App::new()?.run(terminal);
    ratatui::restore();
    info!("Terminal is restored.");
    result
}
