//NOTE: contains the default window and entry-point and stuff.
mod theme;
mod traits;
mod modules;
//mod app;
use modules::interface::Interface;
use traits::Component;
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
            if key.kind == KeyEventKind::Press{
                match key.code{
                    KeyCode::Char('q') => break,
                    KeyCode::Down | KeyCode::Char('j')=>{
                        interface.next();
                    }
                    KeyCode::Up | KeyCode::Char('k')=>{
                        interface.prev();
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
