mod theme; mod icons; mod traits; mod modules; mod config; mod allio;
use color_eyre::eyre::{Result, WrapErr};
use ratatui::{
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
        event::{EnableMouseCapture, DisableMouseCapture},
    },
};
fn main() -> Result<()> {
    color_eyre::install()?;
    let mut stdout = std::io::stdout();
    enable_raw_mode().wrap_err("enable raw mode")?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).wrap_err("enter alternate screen")?;
    let term = ratatui::init();
    let result = allio::run(term);
    let mut stdout = std::io::stdout();
    ratatui::restore();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture).ok();
    disable_raw_mode().ok();
    result
}

