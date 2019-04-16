use crate::Result;
pub use tui::backend::CrosstermBackend;

pub fn create_terminal_backend() -> Result<CrosstermBackend> {
    let alternate_screen = crossterm::AlternateScreen::to_alternate(true)?;
    let backend = CrosstermBackend::with_alternate_screen(alternate_screen)?;
    Ok(backend)
}
