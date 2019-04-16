use crate::core;

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

impl From<core::LoadOption> for Item {
    fn from(load_option: core::LoadOption) -> Self {
        Self {
            load_option: load_option,
        }
    }
}
