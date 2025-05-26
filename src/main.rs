mod app;
mod help;
mod page;
mod pages;
mod store;
mod task;
mod task_form;
mod toml;
mod ui;
mod widgets;

use app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
