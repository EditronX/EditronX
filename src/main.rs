use std::io::{stdout, Result};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::backend::CrosstermBackend;

mod app;
mod buffer;
mod editor;
mod enums;
mod functions;
mod mode_handlers;
mod parser;
mod settings;
mod tab;
mod ui;

fn main() -> Result<()> {
    enable_raw_mode()?;

    execute!(
        stdout(),
        crossterm::terminal::EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = tui::Terminal::new(backend)?;

    let mut app = app::App::new();
    app.run(&mut terminal)?;

    execute!(
        stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    disable_raw_mode()?;

    Ok(())
}
