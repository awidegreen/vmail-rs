extern crate diesel;
extern crate vmail_rs;

use vmail_rs::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    //use vmail_rs::schema::accounts::dsl::*;

    //let connection = establish_connection();

    //let results: QueryResult<Vec<Account>> = accounts.load(&connection);
    //let results = accounts::table
    //.select((accounts::id, (accounts::username,)))
    //.load::<Account>(&connection)
    //.expect("Unable to select.");

    //let results = accounts
    //.filter(enabled.eq(true))
    //.load::<Account>(&connection)
    //.expect("Error loading accounts");

    //println!("Displaying {} accounts", results.len());

    //for acc in results {
    //println!("Username: {}", acc.username);
    //}
}
