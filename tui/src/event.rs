//! App events.

/// The app events.
pub enum Event {
    /// App exit is requested.
    Quit,
    /// Up is pressed.
    Up,
    /// Down is pressed.
    Down,
    /// The enter key is pressed.
    Enter,

    /// Some other unsupported (read uninteresting) event has occured.
    Unsupported,
}
