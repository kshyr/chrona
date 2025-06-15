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
use std::{env, fs::File};
use ticktick::TickTickClient;
use tracing::{debug, error, info};
use tracing_subscriber::{fmt::layer, layer::SubscriberExt as _, util::SubscriberInitExt as _};

fn main() -> Result<()> {
    dotenv().ok();
    let log_file = File::create("chrona.log")?;
    tracing_subscriber::registry()
        .with(layer().with_writer(log_file))
        .init();

    let username = env::var("TICKTICK_USERNAME")?;
    let password = env::var("TICKTICK_PASSWORD")?;
    let ticktick_client = TickTickClient::new(username, password)?;
    info!("TickTick client created successfully.");

    match ticktick_client.get_projects() {
        Some(projects) => debug!("Projects: {:?}", projects),
        None => error!("Error fetching projects"),
    };

    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
