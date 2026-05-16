//NOTE: contains the default window and entry-point and stuff.
mod theme;
mod icons;
mod traits;
mod modules;
use modules::interface::{Interface, InterfaceMode};
use std::time::Duration;
use traits::Component;
use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    crossterm::{event::{self, Event, KeyEvent, KeyCode}}
};

pub enum EventTriggers{
    Key(KeyEvent),
    Tick
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let term = ratatui::init();
    let result = run(term);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut interface = Interface::new();
    loop{
        terminal.draw(|frame|{
            let screen = frame.area();
            interface.set_active(true);//sets the module to active.
            if interface.is_active(){
                interface.render(frame, screen);
            }
        })?;
        if let Event::Key(key) = event::read()?{
            if key.code == KeyCode::Char('q') && interface.mode == InterfaceMode::Normal{
                break;
            }
            interface.event_handler(key);
        }

    }
    Ok(())
}
