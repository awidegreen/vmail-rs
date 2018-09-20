use diesel;
use failure::Error;
use std::result;

#[derive(Debug, Fail)]
pub enum VmailError {
    #[fail(display = "Diesel Error: {}", _0)]
    DieselError(#[cause] diesel::result::Error),
    #[fail(display = "Entry not found: {}", _0)]
    NotFound(String),
}

pub type Result<T> = result::Result<T, Error>;
