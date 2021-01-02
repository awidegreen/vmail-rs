#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate dotenv;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;

use database::connect;
pub use database::DatabaseConnection;
use dotenv::dotenv;
use std::env;

mod database;
pub mod result;

pub mod account;
pub mod alias;
pub mod domain;
pub mod schema;
pub mod tlspolicy;

pub fn establish_connection() -> DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    connect(&database_url)
}
