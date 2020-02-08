#![warn(rust_2018_idioms)]
#![warn(clippy::all)]

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

    pub fn load_options(&mut self) -> impl Iterator<Item = Result<LoadOption>> + '_ {
        self.adapter.load_options()
    }

    pub fn reboot_into(&mut self, num: u16) -> Result<()> {
        self.adapter
            .set_boot_next(num)
            .map_err(error::RebootIntoErrorKind::SetBootNextError)?;
        simplereboot::reboot()
            .map_err(Error::from)
            .map_err(error::RebootIntoErrorKind::RebootError)?;
        Ok(())
    }
}
