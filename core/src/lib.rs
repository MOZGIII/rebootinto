#![warn(rust_2018_idioms)]

use efibootnext::Adapter;
pub use efibootnext::LoadOption;
pub use failure::Error;

mod error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Backend {
    adapter: Adapter,
}

impl Backend {
    pub fn init() -> Result<Self> {
        Ok(Self {
            adapter: Adapter::default(),
        })
    }

    pub fn load_options<'a>(&'a mut self) -> impl Iterator<Item = Result<LoadOption>> + 'a {
        self.adapter.load_options()
    }

    pub fn reboot_into(&mut self, num: u16) -> Result<()> {
        self.adapter
            .set_boot_next(num)
            .map_err(error::RebootIntoErrorKind::SetBootNextError)?;
        simplereboot::reboot()
            .map_err(|e| Error::from(e))
            .map_err(error::RebootIntoErrorKind::RebootError)?;
        Ok(())
    }
}
