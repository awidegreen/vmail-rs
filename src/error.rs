use failure;
use diesel;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Diesel Error: {}", _0)]
    DieselError(diesel::result::Error),
}
