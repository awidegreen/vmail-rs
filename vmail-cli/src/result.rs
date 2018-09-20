use failure::Error;
use std::result;
use vmail_lib::result::VmailError;

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "vmail-lib Error: {}", _0)]
    VmailLib(#[cause] VmailError),
}

pub type Result<T> = result::Result<T, Error>;
