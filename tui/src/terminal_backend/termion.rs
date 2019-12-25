use crate::Result;

use std::io;

use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;

pub struct Termion {
    inner: TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>,
}

impl Backend for Termion {
    fn new() -> Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let inner = TermionBackend::new(stdout);
        let backend = Self { inner };
        Ok(backend)
    }
}

delegate_backend_impl!(Crossterm, self, self.inner);
