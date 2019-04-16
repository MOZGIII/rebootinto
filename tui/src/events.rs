use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Event;
use termion::input::TermRead;

pub struct Events {
    rx: mpsc::Receiver<Event>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.events() {
                    let evt = evt.unwrap();
                    tx.send(evt).unwrap();
                }
            })
        };
        Events {
            rx,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
