use crate::modules::interface::{Interface, InterfaceMode};
use std::{
    sync::mpsc,
    time::Duration,
    thread,
    os::unix::net::UnixStream,
    io::{BufRead, BufReader},
};
use crate::traits::Component;
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyEvent, KeyCode, MouseEvent},
};

pub enum EventTriggers{
    Key(KeyEvent),
    Mouse(MouseEvent),
    Redraw, // daemon, screen resizing and other events
}

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut interface = Interface::new();
    let (tx, rx) = mpsc::channel();
    let tx_input = tx.clone();

    thread::spawn(move || {
        loop {
            match event::read() {
                Ok(Event::Key(key)) => {
                    if tx_input.send(EventTriggers::Key(key)).is_err() { break; }
                }
                Ok(Event::Mouse(mouse)) => {
                    if tx_input.send(EventTriggers::Mouse(mouse)).is_err() { break; }
                }
                Ok(Event::Resize(_, _)) => {
                    if tx_input.send(EventTriggers::Redraw).is_err() { break; }
                }
                Err(_) => {}
                _ => {}
            }
        }
    });

    let tx_daemon = tx.clone();
    thread::spawn(move || {
        let sock = "/tmp/scoutd/scoutd.sock";
        loop {
            match UnixStream::connect(sock) {
                Ok(stream) => {
                    let mut reader = BufReader::new(stream);
                    let mut line = String::new();
                    while let Ok(bytes) = reader.read_line(&mut line) {
                        if bytes == 0 { break; }
                        let _ = tx_daemon.send(EventTriggers::Redraw);
                        line.clear();
                    }
                }
                Err(_) => {
                    thread::sleep(Duration::from_secs(2));
                }
            }
        }
    });

    loop {
        terminal.draw(|frame| {
            interface.set_active(true);
            if interface.is_active() {
                interface.render(frame, frame.area());
            }
        })?;

        match rx.recv()? {
            EventTriggers::Key(key) => {
                if key.code == KeyCode::Char('q') && interface.mode == InterfaceMode::Normal { break; }
                interface.event_handler(key);
            }
            EventTriggers::Mouse(mouse) => {
                interface.theme.theme_refresh();
                interface.mouse_handler(mouse);
            }
            EventTriggers::Redraw => {
                interface.theme.theme_refresh();
            }
        }
    }

    Ok(())
}
