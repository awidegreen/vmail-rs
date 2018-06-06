extern crate diesel;
extern crate vmail_rs;

use vmail_rs::*;

use account::Account;

fn main() {

    let connection = establish_connection();

    let accs = Account::all(&connection);

    for acc in accs {
        println!("Username: {}", acc.username);
    }
}
