//! The terminal backend implementation via [`termion`].

use super::Backend;
use ratatui::backend::TermionBackend;
use std::io;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

/// The [`termion`] backend.
pub struct Termion {
    /// The underlying implementation handle.
    inner: TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>,
}

impl Backend for Termion {
    fn new() -> Result<Self, anyhow::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = stdout.into_alternate_screen()?;
        let inner = TermionBackend::new(stdout);
        let backend = Self { inner };
        Ok(backend)
    }
}

delegate_backend_impl!(Termion, self => self.inner);
