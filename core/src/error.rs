use failure_derive::Fail;

#[derive(Fail, Debug)]
pub enum RebootIntoErrorKind {
    #[fail(display = "set BootNext error: {}", _0)]
    SetBootNextError(failure::Error),

    #[fail(display = "reboot error: {}", _0)]
    RebootError(failure::Error),
}
