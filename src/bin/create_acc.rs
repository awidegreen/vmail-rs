extern crate diesel;
extern crate vmail_rs;

use vmail_rs::*;

use account::{Account, NewAccount};

fn main() {
    let acc = NewAccount {
        username: "aw",
        domain: "mytld",
        password: "nopw",
        quota: None,
        enabled: Some(false),
        sendonly: Some(false),
    };

    let connection = establish_connection();

    Account::create(&connection, acc);

    println!("Account created!");

    //let acc = accounts.find();

    println!("Account deleted");

}
