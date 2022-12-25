//! The terminal backend utility macros.

/// Delegate the implementation of the [`tui`] backend.
#[macro_export]
macro_rules! delegate_backend_impl {
    ($impl_for:ty, $this:ident => $to:expr) => {
        use tui::backend::Backend as TuiBackend;
        use tui::buffer::Cell;
        use tui::layout::Rect;

        impl TuiBackend for $impl_for {
            fn draw<'a, I>(&mut $this, content: I) -> io::Result<()>
            where
                I: Iterator<Item = (u16, u16, &'a Cell)>,
            {
                TuiBackend::draw(&mut $to, content)
            }
            fn hide_cursor(&mut $this) -> io::Result<()> {
                TuiBackend::hide_cursor(&mut $to)
            }
            fn show_cursor(&mut $this) -> io::Result<()> {
                TuiBackend::show_cursor(&mut $to)
            }
            fn get_cursor(&mut $this) -> io::Result<(u16, u16)> {
                TuiBackend::get_cursor(&mut $to)
            }
            fn set_cursor(&mut $this, x: u16, y: u16) -> io::Result<()> {
                TuiBackend::set_cursor(&mut $to, x, y)
            }
            fn clear(&mut $this) -> io::Result<()> {
                TuiBackend::clear(&mut $to)
            }
            fn size(&$this) -> io::Result<Rect> {
                TuiBackend::size(&$to)
            }
            fn flush(&mut $this) -> io::Result<()> {
                TuiBackend::flush(&mut $to)
            }
        }
    };
}
