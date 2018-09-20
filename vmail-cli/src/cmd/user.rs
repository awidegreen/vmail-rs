use clap::ArgMatches;
use crypt::{hash, PasswordScheme};
use rpassword;
use std::process;
use vmail_lib;
use vmail_lib::account::{Account, NewAccount};
use vmail_lib::alias::Alias;
use vmail_lib::domain::Domain;
use vmail_lib::result::Result;

use utils;

fn query_for_password() -> Option<String> {
    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    let pass_confirm = rpassword::prompt_password_stdout("Confirm Password: ").unwrap();

    if pass != pass_confirm {
        eprintln!("Sorry, passwords do not match unable to proceed.");
        return None;
    }

    return Some(pass);
}

fn show(matches: &ArgMatches) -> Result<()> {
    let username = matches.value_of("USER").unwrap_or("");
    let domain = matches.value_of("domain");

    let conn = vmail_lib::establish_connection();

    let accounts = if let Some(domain) = domain {
        let domain = Domain::get(&conn, &domain)?;
        Account::all_by_domain(&conn, &domain)?
    } else {
        Account::all(&conn)?
    };

    for acc in accounts
        .iter()
        .filter(|a| username.is_empty() || a.username == username)
    {
        println!("{}\n", &acc);
    }

    Ok(())
}

fn add(matches: &ArgMatches) -> Result<()> {
    let enabled = !matches.is_present("disabled");
    let sendonly = matches.is_present("sendonly");
    let username = matches.value_of("USER").unwrap();
    let domain = matches.value_of("DOMAIN").unwrap_or("");
    let quota = value_t!(matches.value_of("quota"), i32).unwrap_or_else(|e| {
        eprintln!("Argument 'quota' has to be >= 0");
        e.exit()
    });

    let conn = vmail_lib::establish_connection();

    if !Domain::exsits(&conn, domain)? {
        return Err(format_err!(
            "Unable to create user '{}' as the given domain '{}' does not exist!",
            username,
            domain
        ));
    }

    let pass = query_for_password().unwrap_or_else(|| process::exit(1));

    let pass = hash(PasswordScheme::SHA512_CRYPT, pass).unwrap();

    let a = NewAccount {
        username: username,
        domain: domain,
        password: &pass,
        quota: Some(quota),
        enabled: Some(enabled),
        sendonly: Some(sendonly),
    };

    Account::create(&conn, a)?;

    // TODO add user if it doesn't exists
    println!("User account '{}@{}' has been added!", username, domain);

    Ok(())
}

fn remove(matches: &ArgMatches) -> Result<()> {
    let username = matches.value_of("USER").unwrap();
    let domain = matches.value_of("DOMAIN").unwrap();
    let verbose = matches.is_present("verbose");
    let force = matches.is_present("force");
    let m = format!(
        "Shall the user account '{}@{}' and all associated aliases be removed? (y/N):",
        username, domain
    );

    let conn = vmail_lib::establish_connection();
    let acc = Account::get(&conn, username, domain)?;

    if !force && utils::yes_no(&m, utils::YesNoAnswer::NO) == utils::YesNoAnswer::NO {
        println!(
            "Cancel removing user account '{}@{}' and all associated aliases?",
            username, domain
        );
        return Ok(());
    }

    // delete all aliases
    if let Ok(aliases) = Alias::all_by_dest_account(&conn, &acc) {
        for alias in aliases {
            if verbose {
                println!("Delete alias: {}", alias);
            }
            Alias::delete(&conn, alias)?;
        }
    }

    Account::delete(&conn, acc)?;

    println!(
        "User account '{}@{}' and all its aliases has been removed!",
        username, domain
    );

    Ok(())
}

fn password(matches: &ArgMatches) -> Result<()> {
    let username = matches.value_of("USER").unwrap();
    println!("Set password for '{}'", username);
    let pass = query_for_password().unwrap_or_else(|| process::exit(1));

    println!("Password for user account '{}' has been changed!", username);
    Ok(())
}

fn edit(matches: &ArgMatches) -> Result<()> {
    let username = matches.value_of("USER").unwrap();

    Err(format_err!("Edit not implemented, sorry!"))
}

pub fn dispatch(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("show", Some(m)) => show(m),
        ("add", Some(m)) => add(m),
        ("remove", Some(m)) => remove(m),
        ("password", Some(m)) => password(m),
        ("edit", Some(m)) => edit(m),
        _ => show(matches),
    }
}
