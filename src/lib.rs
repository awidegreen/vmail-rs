#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use models::{Account, Domain};

pub mod schema;
pub mod models;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn accounts() -> Vec<Account> {
    use schema::accounts::dsl::*;

    let conn = establish_connection();

    accounts
        .load::<Account>(&conn)
        .expect("Error loading accounts")
}
pub fn accounts_with_domain(domain_name: &str) -> Vec<Account> {
    use schema::accounts::dsl::*;

    let conn = establish_connection();

    accounts
        .filter(domain.eq(domain_name))
        .load::<Account>(&conn)
        .expect("Error loading accounts")
}

pub fn domains() -> Vec<Domain> {
    use schema::domains::dsl::*;

    let conn = establish_connection();

    domains
        .load::<Domain>(&conn)
        .expect("Error loading domains")
}
