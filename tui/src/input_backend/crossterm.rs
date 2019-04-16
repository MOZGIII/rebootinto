use super::InputBackend;
use crate::event::Event;
use crossterm::{InputEvent, KeyEvent};

pub struct CrosstermInputBackend<I>
where
    I: Iterator<Item = InputEvent>,
{
    iter: Box<I>,
}

impl<I> Iterator for CrosstermInputBackend<I>
where
    I: Iterator<Item = InputEvent>,
{
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let evt = match self.iter.next() {
            None => return None,
            Some(evt) => evt,
        };
        Some(match evt {
            InputEvent::Keyboard(KeyEvent::Char('q')) => Event::Quit,
            InputEvent::Keyboard(KeyEvent::Down) => Event::Down,
            InputEvent::Keyboard(KeyEvent::Up) => Event::Up,
            InputEvent::Keyboard(KeyEvent::Char('\n')) => Event::Enter,
            _ => Event::Unsupported,
        })
    }
}

impl<I> InputBackend for CrosstermInputBackend<I> where I: Iterator<Item = InputEvent> {}

#[cfg(not(feature = "crossterm_backend_sync_input"))]
pub mod async_reader {
    use super::CrosstermInputBackend;

    pub fn create_input_backend() -> CrosstermInputBackend<crossterm::AsyncReader> {
        CrosstermInputBackend {
            iter: Box::new(crossterm::input().read_async()),
        }
    }

    #[test]
    fn casts_to_input_backend() {
        use super::InputBackend;
        let mut concrete = create_input_backend();
        let _backend: &mut InputBackend = &mut concrete;
    }
}

#[cfg(not(feature = "crossterm_backend_sync_input"))]
pub use self::async_reader::*;

#[cfg(feature = "crossterm_backend_sync_input")]
pub mod sync_reader {
    use super::CrosstermInputBackend;

    pub fn create_input_backend() -> CrosstermInputBackend<crossterm::SyncReader> {
        CrosstermInputBackend {
            iter: Box::new(crossterm::input().read_sync()),
        }
    }

    #[test]
    fn casts_to_input_backend() {
        use super::InputBackend;
        let mut concrete = create_input_backend();
        let _backend: &mut InputBackend = &mut concrete;
    }
}

#[cfg(feature = "crossterm_backend_sync_input")]
pub use self::sync_reader::*;
