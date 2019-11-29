use crate::Result;

use std::io;

pub use tui::backend::CrosstermBackend;

pub fn create_terminal_backend() -> Result<CrosstermBackend<io::Stdout>> {
    let alternate_screen = crossterm::screen::AlternateScreen::to_alternate(true)?;
    let backend = CrosstermBackend::with_alternate_screen(io::stdout(), alternate_screen)?;
    Ok(backend)
}
