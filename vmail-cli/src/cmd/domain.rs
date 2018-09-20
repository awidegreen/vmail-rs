use clap::ArgMatches;
use result::Result;
use vmail_lib;
use vmail_lib::account::Account;
use vmail_lib::alias::Alias;
use vmail_lib::domain::{Domain, NewDomain};

use utils;

fn show(matches: &ArgMatches) -> Result<()> {
    let domain = matches.value_of("DOMAIN");
    let with_users = matches.is_present("with-users");
    let with_aliases = matches.is_present("with-aliases");

    let conn = vmail_lib::establish_connection();

    let domains = match domain {
        Some(domain) => vec![Domain::get(&conn, domain)?],
        _ => Domain::all(&conn)?,
    };

    for d in domains {
        println!("{}", d);
        if with_users {
            if let Ok(accounts) = Account::all_by_domain(&conn, &d) {
                for a in accounts {
                    println!("  {}@{}", a.username, a.domain);
                    if with_aliases {
                        if let Ok(aliases) = Alias::all_by_dest_account(&conn, &a) {
                            for alias in aliases {
                                println!("    {}", alias);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn add(matches: &ArgMatches) -> Result<()> {
    let domain = matches.value_of("DOMAIN").unwrap();

    let conn = vmail_lib::establish_connection();
    if Domain::exsits(&conn, domain)? {
        return Err(format_err!(
            "The domain '{}' already exists, unable to add!",
            domain
        ));
    }
    let d = NewDomain {
        domain: String::from(domain),
    };

    Domain::create(&conn, d)?;

    println!("Domain '{}' has been added!", domain);
    Ok(())
}

fn remove(matches: &ArgMatches) -> Result<()> {
    let domain_s = matches.value_of("DOMAIN").unwrap();
    let force = matches.is_present("force");
    let verbose = matches.is_present("verbose");

    let conn = vmail_lib::establish_connection();

    let domain = Domain::get(&conn, domain_s)?;

    let m = format!(
        "Shall the domain '{}' and all related user accounts really be removed? (y/N):",
        domain
    );

    if !force && utils::yes_no(&m, utils::YesNoAnswer::NO) == utils::YesNoAnswer::NO {
        println!("Canceled removing domain '{}'", domain);
        return Ok(());
    }

    if let Ok(accounts) = Account::all_by_domain(&conn, &domain) {
        for acc in accounts {
            if verbose {
                println!("Delete user: {}@{}", acc.username, acc.domain);
            }
            for alias in Alias::all_by_dest_account(&conn, &acc)? {
                if verbose {
                    println!("  Delete alias: {}", alias);
                }
                Alias::delete(&conn, alias)?;
            }
            Account::delete(&conn, acc)?;
        }
    }
    Domain::delete(&conn, domain)?;

    println!(
        "Domain '{}' and all its user accounts has been removed!",
        domain_s
    );
    return Ok(());
}

pub fn dispatch(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("show", Some(m)) => show(m),
        ("add", Some(m)) => add(m),
        ("remove", Some(m)) => remove(m),
        _ => show(matches),
    }
}
