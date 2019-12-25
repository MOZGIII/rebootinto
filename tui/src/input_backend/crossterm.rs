use super::InputBackend;
use crate::event::Event;
use crossterm::event::{read, Event as CrosstermEvent, KeyCode, KeyEvent};

pub struct CrosstermInputBackend;

impl Iterator for CrosstermInputBackend {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let evt = match read() {
            Err(_) => return None,
            Ok(evt) => evt,
        };
        Some(match evt {
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            })
            | CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => Event::Quit,
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => Event::Down,
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => Event::Up,
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Char('\n'),
                ..
            })
            | CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => Event::Enter,
            _ => Event::Unsupported,
        })
    }
}

impl InputBackend for CrosstermInputBackend {}

pub fn create_input_backend() -> CrosstermInputBackend {
    CrosstermInputBackend
}

#[test]
fn casts_to_input_backend() {
    use super::InputBackend;
    let mut concrete = create_input_backend();
    let _backend: &mut dyn InputBackend = &mut concrete;
}
