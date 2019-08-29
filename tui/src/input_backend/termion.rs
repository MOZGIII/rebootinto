use super::InputBackend;
use crate::event::Event;
use std::io;
use termion::event::{Event as TermionEvent, Key};
use termion::input::{Events, TermRead};

pub struct TermionInputBackend {
    iter: Events<io::Stdin>,
}

impl InputBackend for TermionInputBackend {}

impl Iterator for TermionInputBackend {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let evt = match self.iter.next() {
            None => return None,
            Some(Ok(evt)) => evt,
            Some(Err(_)) => return None,
        };
        Some(match evt {
            TermionEvent::Key(Key::Char('q')) => Event::Quit,
            TermionEvent::Key(Key::Down) => Event::Down,
            TermionEvent::Key(Key::Up) => Event::Up,
            TermionEvent::Key(Key::Char('\n')) => Event::Enter,
            _ => Event::Unsupported,
        })
    }
}

pub fn create_input_backend() -> TermionInputBackend {
    TermionInputBackend {
        iter: io::stdin().events(),
    }
}

#[test]
fn casts_to_input_backend() {
    let mut concrete = create_input_backend();
    let _backend: &mut dyn InputBackend = &mut concrete;
}
