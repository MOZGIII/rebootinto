#[macro_use]
extern crate failure_derive;

use efibootnext;
use efivar;

pub use efibootnext::LoadOption;
pub use failure::Error;

mod error;

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

    pub fn load_options<'a>(&'a mut self) -> impl Iterator<Item = Result<LoadOption>> + 'a {
        efibootnext::load_options(self.var_manager.as_mut())
    }

    pub fn reboot_into(&mut self, num: u16) -> Result<()> {
        efibootnext::set_boot_next(self.var_manager.as_mut(), num)
            .map_err(error::RebootIntoErrorKind::SetBootNextError)?;
        simplereboot::reboot()
            .map_err(|e| Error::from(e))
            .map_err(error::RebootIntoErrorKind::RebootError)?;
        Ok(())
    }
}
