#[macro_use]
extern crate failure_derive;

use efibootnext;
use efivar;

pub use efibootnext::{LoadOption, LoadOptionIter};
pub use failure::Error;

mod error;
mod reboot;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Backend {
    var_manager: Box<dyn efivar::VarManager>,
}

impl Backend {
    pub fn init() -> Result<Self> {
        Ok(Self {
            var_manager: efivar::system(),
        })
    }

    pub fn load_options(&mut self) -> LoadOptionIter {
        efibootnext::load_options(self.var_manager.as_mut())
    }

    pub fn reboot_into(&mut self, num: u16) -> Result<()> {
        efibootnext::set_boot_next(self.var_manager.as_mut(), num)
            .map_err(error::RebootIntoErrorKind::SetBootNextError)?;
        reboot::reboot().map_err(error::RebootIntoErrorKind::RebootError)?;
        Ok(())
    }
}
