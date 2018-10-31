use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use result::Result;
use vmail_lib::account::Account;
use vmail_lib::alias::Alias;
use vmail_lib::domain::{Domain, NewDomain};
use vmail_lib::{establish_connection, DatabaseConnection};

use utils;

//------------------------------------------------------------------------------

fn show(matches: &ArgMatches, conn: DatabaseConnection) -> Result<()> {
    let domain = matches.value_of("DOMAIN");
    let with_users = matches.is_present("with-users");
    let with_aliases = matches.is_present("with-aliases");

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

//------------------------------------------------------------------------------

fn add(matches: &ArgMatches, conn: DatabaseConnection) -> Result<()> {
    let domain = matches.value_of("DOMAIN").unwrap();

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

//------------------------------------------------------------------------------

fn remove(matches: &ArgMatches, conn: DatabaseConnection) -> Result<()> {
    let domain_s = matches.value_of("DOMAIN").unwrap();
    let force = matches.is_present("force");
    let verbose = matches.is_present("verbose");

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
                Alias::delete(&conn, &alias)?;
            }
            Account::delete(&conn, &acc)?;
        }
    }
    Domain::delete(&conn, &domain)?;

    println!(
        "Domain '{}' and all its user accounts has been removed!",
        domain_s
    );

    Ok(())
}

//------------------------------------------------------------------------------

pub fn dispatch(matches: &ArgMatches) -> Result<()> {
    let conn = establish_connection();

    match matches.subcommand() {
        ("show", Some(m)) => show(m, conn),
        ("add", Some(m)) => add(m, conn),
        ("remove", Some(m)) => remove(m, conn),
        _ => show(matches, conn),
    }
}

//------------------------------------------------------------------------------

pub fn get_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("domain")
        .about("Manage domains for a vmail database")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("show")
                .about("Show domains")
                .aliases(&["list", "ls"])
                .arg(Arg::with_name("DOMAIN").help("Filter on domain"))
                .arg(
                    Arg::with_name("with-users")
                        .long("with-users")
                        .short("u")
                        .help("Show all users for the domain"),
                ).arg(
                    Arg::with_name("with-aliases")
                        .long("with-aliases")
                        .short("a")
                        .requires("with-users")
                        .help("Show all aliases for the users"),
                ),
        ).subcommand(
            SubCommand::with_name("add")
                .about("Add a new domain to the database")
                .arg(
                    Arg::with_name("DOMAIN")
                        .required(true)
                        .help("The domain name which should be added."),
                ),
        ).subcommand(
            SubCommand::with_name("remove")
                .about(
                    "Remove a domain from the database, this will also delete all related users.",
                ).aliases(&["rm", "delete"])
                .arg(
                    Arg::with_name("force")
                        .long("force")
                        .short("f")
                        .help("Force the deleting the given domain"),
                ).arg(
                    Arg::with_name("verbose")
                        .long("verbose")
                        .short("v")
                        .help("Verbose output what has been deleted"),
                ).arg(
                    Arg::with_name("DOMAIN")
                        .required(true)
                        .help("The domain name which should be deleted."),
                ),
        )
}
