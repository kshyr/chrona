mod app;
mod auth;
mod help;
mod page;
mod pages;
mod store;
mod task;
mod ticktick;
mod toml;
mod ui;
mod widgets;

use std::fs::File;

use anyhow::Result;
use app::App;
use dotenv::dotenv;
use oauth2::TokenResponse as _;
use ticktick::TickTickClient;
use tracing::{debug, error, info};
use tracing_subscriber::{fmt::layer, layer::SubscriberExt as _, util::SubscriberInitExt as _};

fn main() -> Result<()> {
    dotenv().ok();
    let log_file = File::create("chrona.log")?;
    tracing_subscriber::registry()
        .with(layer().with_writer(log_file))
        .init();

    let access_token = match auth::load_token() {
        Ok(token) => {
            info!("Successfully loaded cached token.");
            token
        }
        Err(e) => {
            info!(
                "Could not load cached token ({}), starting interactive auth.",
                e
            );
            match auth::run_interactive_auth() {
                Ok(token_response) => {
                    let token = token_response.access_token().clone();
                    if let Err(save_err) = auth::save_token(token_response) {
                        error!("Warning: Could not save new access token: {}", save_err);
                    }
                    token
                }
                Err(auth_err) => {
                    error!("Fatal: TickTick authorization failed: {}", auth_err);
                    return Err(auth_err);
                }
            }
        }
    };

    let ticktick_client = TickTickClient::new(access_token)?;
    debug!("TickTick client created successfully.");

    match ticktick_client.get_projects() {
        Ok(projects) => debug!("Projects: {:?}", projects),
        Err(e) => error!("Error fetching projects: {}", e),
    };

    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
