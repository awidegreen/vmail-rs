#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate dotenv;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod result;

pub mod account;
pub mod alias;
pub mod domain;
pub mod schema;
pub mod tlspolicy;

pub type DatabaseConnection = MysqlConnection;

pub fn establish_connection() -> DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    DatabaseConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
