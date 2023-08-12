//! The core implementation of the rebootinto.

#[cfg(target_os = "macos")]
use rebootinto_efibootnext_mock as efibootnext;

pub use efibootnext::LoadOption;

/// The backend providing the rebootinto flow.
pub struct Backend {
    /// The cross-platform adapter to interact with the boot options of the EFI firmware.
    adapter: efibootnext::Adapter,
}

impl Backend {
    /// Run all the necessary initialization and prepare a new [`Self`].
    pub fn init() -> Result<Self, InitError> {
        Ok(Self {
            adapter: efibootnext::Adapter::default(),
        })
    }

    /// Provide an iterator over the available load options.
    pub fn load_options(
        &mut self,
    ) -> Result<impl Iterator<Item = Result<LoadOption, LoadOptionError>> + '_, LoadOptionsError>
    {
        let iter = self
            .adapter
            .load_options()
            .map_err(LoadOptionsError::Efibootnext)?;
        Ok(iter.map(|result| result.map_err(LoadOptionError::Efibootnext)))
    }

    /// Perform the reboot operation into the given load option.
    pub fn reboot_into(&mut self, num: u16) -> Result<(), RebootIntoError> {
        self.adapter
            .set_boot_next(num)
            .map_err(RebootIntoError::SetBootNext)?;
        simplereboot::reboot().map_err(RebootIntoError::Reboot)?;
        Ok(())
    }
}

/// An error that can occur during the initialization.
#[derive(Debug, thiserror::Error)]
pub enum InitError {}

/// An error that can occur at the `load_options` call.
#[derive(Debug, thiserror::Error)]
pub enum LoadOptionsError {
    /// Something went wrong while getting the load option.
    #[error("enumerating load options error: {0}")]
    Efibootnext(efibootnext::error::EnumerateLoadOptionsError),
}

/// An error that can occur at the `load_options` call.
#[derive(Debug, thiserror::Error)]
pub enum LoadOptionError {
    /// Something went wrong while getting the load option.
    #[error("load option error: {0}")]
    Efibootnext(efibootnext::error::GetLoadOptionError),
}

/// An error that can occur at the `reboot_into` call.
#[derive(Debug, thiserror::Error)]
pub enum RebootIntoError {
    /// Something went wrong when setting the `BootNext` EFI variable.
    #[error("set BootNext error: {0}")]
    SetBootNext(efibootnext::error::SetBootNextError),
    /// Something went wrong during the reboot.
    #[error("reboot error: {0}")]
    Reboot(std::io::Error),
}
