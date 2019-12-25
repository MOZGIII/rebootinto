#[cfg(feature = "crossterm_backend")]
mod crossterm;
#[cfg(feature = "crossterm_backend")]
pub use self::crossterm::Crossterm as Impl;

#[cfg(feature = "termion_backend")]
mod termion;
#[cfg(feature = "termion_backend")]
pub use self::termion::Termion as Impl;

use super::Result;

pub trait Backend: tui::backend::Backend {
    fn new() -> Result<Self>
    where
        Self: Sized;
}
