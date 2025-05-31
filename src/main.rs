mod app;
mod help;
mod page;
mod pages;
mod store;
mod task;
mod toml;
mod ui;
mod widgets;

use std::fs::File;

use app::App;
use tracing_subscriber::{fmt::layer, layer::SubscriberExt as _, util::SubscriberInitExt as _};
use tui_logger;

fn main() -> color_eyre::Result<()> {
    tui_logger::init_logger(tui_logger::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(tui_logger::LevelFilter::Trace);

    let log_file = File::create("chrona.log")?;
    tracing_subscriber::registry()
        .with(layer().with_writer(log_file))
        .with(tui_logger::TuiTracingSubscriberLayer)
        .init();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
