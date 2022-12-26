//! The terminal backend.

#[cfg(feature = "crossterm_backend")]
mod crossterm;
#[cfg(feature = "crossterm_backend")]
pub use self::crossterm::Crossterm as Impl;

#[cfg(feature = "termion_backend")]
mod termion;
#[cfg(feature = "termion_backend")]
pub use self::termion::Termion as Impl;

/// The backend.
pub trait Backend: tui::backend::Backend {
    /// Constrct and initializer a new backend.
    fn new() -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}
