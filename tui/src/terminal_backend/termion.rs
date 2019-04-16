use crate::Result;

use std::io;

use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;

pub fn create_terminal_backend(
) -> Result<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    Ok(backend)
}
