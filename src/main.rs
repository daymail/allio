//NOTE: contains the default window and entry-point and stuff.
mod theme;
mod icons;
mod traits;
mod modules;
use modules::interface::{Interface, InterfaceMode};
use std::{
    sync::{mpsc},
    time::Duration,
    thread,
    os::unix::net::UnixStream,
    io::{BufRead, BufReader}
};
use traits::Component;
use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    crossterm::{event::{self, Event, KeyEvent, KeyCode}}
};

pub enum EventTriggers{
    Key(KeyEvent),
    Redraw //daemon, screen resizing and other events
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let term = ratatui::init();
    let result = run(term);
    ratatui::restore();
    result
}


fn run(mut terminal: DefaultTerminal) -> Result<()>{
    let mut interface = Interface::new();
    let (tx, rx) = mpsc::channel();

    let tx_input = tx.clone();
    thread::spawn(move ||  {
        loop{
            match event::read(){
                std::result::Result::Ok(Event::Key(key)) =>{
                    if tx_input.send(EventTriggers::Key(key)).is_err(){break;}
                }
                std::result::Result::Ok(Event::Resize(_,_)) =>{
                    if tx_input.send(EventTriggers::Redraw).is_err() {break;}
                }
                _ => {}
            }
        }
    });

    let tx_daemon = tx.clone();
    thread::spawn(move || {
        let sock = "/tmp/scoutd/scoutd.sock";
        loop{
            match UnixStream::connect(sock){
                std::result::Result::Ok(stream) => {
                    let mut reader = BufReader::new(stream);
                    let mut line = String::new();
                    while let std::result::Result::Ok(bytes) = reader.read_line(&mut line){
                        if bytes == 0 {break;}
                        let _ = tx_daemon.send(EventTriggers::Redraw);
                        line.clear();
                    }
                }
                std::result::Result::Err(_) => {
                    thread::sleep(Duration::from_secs(2));
                    //Daemon is offline.
                }
            }
        }
    });

    loop{
        terminal.draw(|frame| {
            interface.set_active(true);
            if interface.is_active(){
                interface.render(frame, frame.area());
            }
        })?;
        match rx.recv()?{
            EventTriggers::Key(key) =>{
                if key.code == KeyCode::Char('q') && interface.mode == InterfaceMode::Normal{break;}
                interface.event_handler(key);
            }
            EventTriggers::Redraw =>{
                //redraw
            }
        }
    }
    Ok(())
}
