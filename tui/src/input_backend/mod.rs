pub use crate::event::Event;
use std::iter::Iterator;

pub trait InputBackend: Iterator<Item = Event> {}

#[cfg(feature = "crossterm_backend")]
mod crossterm;
#[cfg(feature = "crossterm_backend")]
pub use self::crossterm::*;

#[cfg(feature = "termion_backend")]
mod termion;
#[cfg(feature = "termion_backend")]
pub use self::termion::*;

#[test]
fn create_input_backend_casts_to_input_backend() {
    let mut concrete = self::create_input_backend();
    let _backend: &mut dyn InputBackend = &mut concrete;
}
