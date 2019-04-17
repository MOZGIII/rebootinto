use super::InputBackend;
use crate::event::Event;
use crossterm::{InputEvent, KeyEvent};

pub trait CrosstermInputBackend<I>
where
    I: Iterator<Item = InputEvent>,
{
    fn iter(&mut self) -> &mut I;
}

impl<I> Iterator for CrosstermInputBackend<I>
where
    I: Iterator<Item = InputEvent>,
{
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let evt = match self.iter().next() {
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

impl<T, I> InputBackend for T
where
    T:  CrosstermInputBackend<I>,
    I: Iterator<Item = InputEvent>,
{
}

pub struct CrosstermInputIterator<I>
where
    I: Iterator<Item = InputEvent>,
{
    iter: Box<I>,
}

impl<I> CrosstermInputBackend<I> for CrosstermInputIterator<I>
where
    I: Iterator<Item = InputEvent>,
{
    fn iter(&mut self) -> &mut I {
        self.iter.as_mut()
    }
}

pub mod async_reader {
    pub use super::CrosstermInputBackend;
    pub use super::CrosstermInputIterator;

    pub fn create_input_backend() -> CrosstermInputIterator<crossterm::AsyncReader> {
        CrosstermInputIterator {
            iter: Box::new(crossterm::input().read_async()),
        }
    }

    #[cfg(test)]
    fn casts_to_input_backend() {
        use super::InputBackend;
        let mut concrete = create_input_backend();
        let _backend: &mut InputBackend = &mut concrete;
    }
}

pub mod sync_reader {
    pub use super::CrosstermInputBackend;
    pub use super::CrosstermInputIterator;

    pub fn create_input_backend() -> CrosstermInputIterator<crossterm::SyncReader> {
        CrosstermInputIterator {
            iter: Box::new(crossterm::input().read_sync()),
        }
    }

    #[cfg(test)]
    fn casts_to_input_backend() {
        use super::InputBackend;
        let mut concrete = create_input_backend();
        let _backend: &mut InputBackend = &mut concrete;
    }
}
