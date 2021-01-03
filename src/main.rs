#[macro_use]
extern crate clap;
extern crate rpassword;
extern crate vmail_lib;
#[macro_use]
extern crate failure;
extern crate dotenv;
extern crate sha_crypt;

use std::io;
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
        ("completions", Some(m)) => {
            let shell = m.value_of("SHELL").unwrap();
            cli::build_cli().gen_completions_to(
                "vmail-cli",
                shell.parse().unwrap(),
                &mut io::stdout(),
            );
            Ok(())
        }
        //("policies", Some(m)) => user(m),
        _ => Err(format_err!("Subcommand not implemented")),
    };

    if let Err(e) = r {
        eprintln!("ERROR: {}", e);
        process::exit(-1)
    }
}
