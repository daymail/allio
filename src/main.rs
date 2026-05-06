//NOTE: contains the default window and entry-point and stuff.
mod component;
mod modules;
//mod app;
use modules::splash::Splash;
use component::Component;
use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    crossterm::{event::{self, Event}}
};
use crossterm::event::{KeyEventKind, KeyCode};
fn main() -> Result<()> {
    color_eyre::install()?;
    let term = ratatui::init();
    let result = run(term);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop{
        let mut splash_m = Splash::new();
        terminal.draw(|frame|{
            let screen = frame.area();
            splash_m.set_active(true);//sets the module to active.
            if splash_m.is_active(){
                splash_m.render(frame, screen);
            }
        })?;
        if let Event::Key(key) = event::read()?{
            if key.kind == KeyEventKind::Press{
                if key.code == KeyCode::Char('q'){
                    break;
                }
            }
        }
    }
    Ok(())
}
