pub use crate::event::Event;
use std::iter::Iterator;

pub trait InputBackend: Iterator<Item = Event> {}

#[cfg(feature = "crossterm_backend")]
mod crossterm;
#[cfg(feature = "crossterm_backend")]
pub use self::crossterm::async_reader::*;

#[cfg(feature = "termion_backend")]
mod termion;
#[cfg(feature = "termion_backend")]
pub use self::termion::*;
