use crate::core;

use tui::widgets::Text;

#[derive(Debug)]
pub struct Item {
    load_option: core::LoadOption,
}

impl Item {
    pub fn into_inner(self) -> core::LoadOption {
        self.load_option
    }
}

impl AsRef<str> for Item {
    fn as_ref(&self) -> &str {
        self.load_option.description.as_ref()
    }
}

impl<'a> Into<Text<'a>> for &'a Item {
    fn into(self) -> Text<'a> {
        Text::raw(self.as_ref())
    }
}

impl From<core::LoadOption> for Item {
    fn from(load_option: core::LoadOption) -> Self {
        Self { load_option }
    }
}
