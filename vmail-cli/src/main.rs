#[macro_use]
extern crate clap;
extern crate rpassword;
extern crate vmail_lib;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate rand;
extern crate sha_crypt;

use std::process;

mod cli;
mod cmd;
mod crypt;
mod result;
mod utils;

fn main() {
    let matches = cli::build_cli().get_matches();

    let r = match matches.subcommand() {
        ("user", Some(m)) => cmd::user::dispatch(m),
        ("alias", Some(m)) => cmd::alias::dispatch(m),
        ("domain", Some(m)) => cmd::domain::dispatch(m),
        //("policies", Some(m)) => user(m),
        _ => Err(format_err!("Subcommand not implemented")),
    };

    if let Err(e) = r {
        eprintln!("ERROR: {}", e);
        process::exit(-1)
    }
}
