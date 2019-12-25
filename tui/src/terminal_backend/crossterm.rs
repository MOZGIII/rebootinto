use super::Backend;
use crate::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Command, ErrorKind,
};
use std::io::{self, Write};
use tui::backend::CrosstermBackend;

pub struct Crossterm {
    inner: CrosstermBackend<io::Stdout>,
}

impl Backend for Crossterm {
    fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        typed_execute(&mut stdout, EnterAlternateScreen)?;
        let inner = CrosstermBackend::new(stdout);
        let backend = Self { inner };
        Ok(backend)
    }
}

impl Drop for Crossterm {
    fn drop(&mut self) {
        typed_execute(&mut self.inner, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}

delegate_backend_impl!(Crossterm, self => self.inner);

fn typed_execute<W: Write, C: Command>(
    write: &mut W,
    command: C,
) -> std::result::Result<(), ErrorKind> {
    execute!(write, command)
}
