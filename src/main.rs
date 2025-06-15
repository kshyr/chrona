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

use anyhow::Result;
use app::App;
use dotenv::dotenv;
use std::fs::{self, File};
use tracing_subscriber::{fmt::layer, layer::SubscriberExt as _, util::SubscriberInitExt as _};

fn main() -> Result<()> {
    dotenv().ok();
    fs::create_dir_all(".cache")?;
    let log_file = File::create(".cache/chrona.log")?;
    tracing_subscriber::registry()
        .with(layer().with_writer(log_file))
        .init();

    let terminal = ratatui::init();
    let result = App::new()?.run(terminal);
    ratatui::restore();
    result
}
