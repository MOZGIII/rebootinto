//! The mock `efibootnext` crate implementation.

/// A mock `Adapter`.
#[derive(Debug, Default)]
pub struct Adapter {
    /// Just a dummy private items to make linters happy.
    _private: (),
}

/// The mock [`LoadOption`] type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadOption {
    /// Number of the load option.
    pub number: u16,
    /// Description of the load option.
    pub description: String,
}

impl std::fmt::Display for LoadOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.description)
    }
}

/// The load options to provide.
const MOCK_ITEMS: &[&str] = &["Ubuntu", "Windows", "Arch", "USB Boot"];

impl Adapter {
    /// Mock `load_options` call.
    pub fn load_options(
        &self,
    ) -> Result<
        impl Iterator<Item = Result<LoadOption, error::GetLoadOptionError>>,
        error::EnumerateLoadOptionsError,
    > {
        Ok(MOCK_ITEMS.iter().copied().enumerate().map(|(num, item)| {
            Ok(LoadOption {
                number: num.try_into().unwrap(),
                description: item.into(),
            })
        }))
    }

    /// Mock `set_boot_next` call.
    pub fn set_boot_next(&self, _num: u16) -> Result<(), error::SetBootNextError> {
        match std::env::var("MOCK_SET_BOOT_ERROR") {
            Ok(val) if val == "true" => Err(error::SetBootNextError),
            _ => Ok(()),
        }
    }
}

/// Mock errors.
pub mod error {
    /// Mock `EnumerateLoadOptionsError`.
    #[derive(Debug, thiserror::Error)]
    #[error("mock EnumerateLoadOptionsError")]
    pub struct EnumerateLoadOptionsError;

    /// Mock `GetLoadOptionError`.
    #[derive(Debug, thiserror::Error)]
    #[error("mock GetLoadOptionError")]
    pub struct GetLoadOptionError;

    /// Mock `SetBootNextError`.
    #[derive(Debug, thiserror::Error)]
    #[error("mock SetBootNextError")]
    pub struct SetBootNextError;
}
