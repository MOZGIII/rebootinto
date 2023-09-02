//! The terminal backend implementation via [`crossterm`].

use super::Backend;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;

/// The [`crossterm`] backend.
pub struct Crossterm {
    /// The underlying implementation handle.
    inner: CrosstermBackend<AlternateScreenWriter<std::io::Stdout>>,
}

impl Backend for Crossterm {
    fn new() -> Result<Self, anyhow::Error> {
        let stdout = std::io::stdout();
        let stdout = AlternateScreenWriter::new(stdout)?;
        let inner = CrosstermBackend::new(stdout);
        let backend = Self { inner };
        Ok(backend)
    }
}

delegate_backend_impl!(Crossterm, self => self.inner);

/// The writer that wraps `W` and manages the alternate screen mode
/// and raw mode.
struct AlternateScreenWriter<W: std::io::Write>(W);

impl<W: std::io::Write> AlternateScreenWriter<W> {
    /// Create a new [`AlternateScreenWriter`] over `w`.
    pub fn new(mut w: W) -> std::io::Result<Self> {
        enable_raw_mode()?;
        execute!(&mut w, EnterAlternateScreen)?;
        Ok(Self(w))
    }
}

impl<W: std::io::Write> Drop for AlternateScreenWriter<W> {
    fn drop(&mut self) {
        execute!(&mut self.0, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}

impl<W: std::io::Write> std::io::Write for AlternateScreenWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::Write::write(&mut self.0, buf)
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        std::io::Write::write_vectored(&mut self.0, bufs)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::Write::flush(&mut self.0)
    }
}
