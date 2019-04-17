use super::InputBackend;
use crate::event::Event;
use crossterm::{InputEvent, KeyEvent};

pub trait CrosstermInputBackend {
    type CrosstermInputIter: Iterator<Item = InputEvent>;

    fn iter(&mut self) -> &mut Self::CrosstermInputIter;
}

impl<I> InputBackend for CrosstermInputBackend<CrosstermInputIter = I> where
    I: Iterator<Item = InputEvent>
{
}

impl<I> Iterator for CrosstermInputBackend<CrosstermInputIter = I>
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

pub struct CrosstermInputIterator<I>
where
    I: Iterator<Item = InputEvent>,
{
    iter: Box<I>,
}

impl<I> CrosstermInputBackend for CrosstermInputIterator<I>
where
    I: Iterator<Item = InputEvent>,
{
    type CrosstermInputIter = I;

    fn iter(&mut self) -> &mut Self::CrosstermInputIter {
        self.iter.as_mut()
    }
}

// FIXME: ideally this should not be required.
impl<I> Iterator for CrosstermInputIterator<I>
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

impl<I> InputBackend for CrosstermInputIterator<I> where I: Iterator<Item = InputEvent> {}

pub mod async_reader {
    pub use super::CrosstermInputBackend;
    pub use super::CrosstermInputIterator;

    pub fn create_input_backend() -> CrosstermInputIterator<crossterm::AsyncReader> {
        CrosstermInputIterator {
            iter: Box::new(crossterm::input().read_async()),
        }
    }
}
