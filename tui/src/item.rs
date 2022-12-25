//! The UI item implementation.

use crate::core;

use tui::widgets::Text;

/// The UI item.
#[derive(Debug)]
pub struct Item {
    /// The load option.
    pub load_option: core::LoadOption,
}

impl AsRef<str> for Item {
    fn as_ref(&self) -> &str {
        self.load_option.description.as_ref()
    }
}

impl<'a> From<&'a Item> for Text<'a> {
    fn from(value: &'a Item) -> Self {
        Text::raw(value.as_ref())
    }
}
