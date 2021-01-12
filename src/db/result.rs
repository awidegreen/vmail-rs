use failure::{Error, Fail};
use std::result;

#[derive(Debug, Fail)]
pub enum VmailError {
    #[fail(display = "Entry not found: {}", _0)]
    NotFound(String),
}

pub type Result<T> = result::Result<T, Error>;
