use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use crypt::{hash, PasswordScheme};
use dotenv::dotenv;
use rpassword;
use std::env;
use std::process;
use vmail_lib::account::{Account, NewAccount};
use vmail_lib::alias::Alias;
use vmail_lib::domain::Domain;
use vmail_lib::result::Result;
use vmail_lib::{establish_connection, DatabaseConnection};

use utils;

const DOMAIN_MISSING: &'static str =
    "A domain has to be provided via user command (arg '-d|--domain') or via '.env' file";

fn query_for_password() -> Option<String> {
    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    if pass.is_empty() {
        eprintln!("Sorry, empty passwords are not allowed!");
        return None;
    }
    let pass_confirm = rpassword::prompt_password_stdout("Confirm Password: ").unwrap();

    if pass != pass_confirm {
        eprintln!("Sorry, passwords do not match, unable to proceed!");
        return None;
    }

    Some(pass)
}

fn show(matches: &ArgMatches, conn: DatabaseConnection, domain: Option<&str>) -> Result<()> {
    let username = matches.value_of("USER").unwrap_or("");
    let verbose = matches.is_present("verbose");

    let accounts = if let Some(domain) = domain {
        let domain = Domain::get(&conn, &domain)?;
        println!("Show accounts for domain: {}", domain);
        Account::all_by_domain(&conn, &domain)?
    } else {
        Account::all(&conn)?
    };

    for acc in accounts
        .iter()
        .filter(|a| username.is_empty() || a.username == username)
    {
        println!("{}", &acc);
        if verbose {
            if let Ok(aliases) = Alias::all_by_dest_account(&conn, acc) {
                println!("Aliases: ");
                for al in aliases {
                    println!(" {}@{} ", al.source_username(), al.source_domain);
                }
            }
        }
        println!("");
    }

    Ok(())
}

fn add(matches: &ArgMatches, conn: DatabaseConnection, domain: Option<&str>) -> Result<()> {
    let enabled = !matches.is_present("disabled");
    let sendonly = matches.is_present("sendonly");
    let username = matches.value_of("USER").unwrap();
    let domain = domain.ok_or_else(|| format_err!("{}", DOMAIN_MISSING))?;

    let quota = value_t!(matches.value_of("quota"), i32).unwrap_or_else(|e| {
        eprintln!("Argument 'quota' has to be >= 0");
        e.exit()
    });

    if !Domain::exsits(&conn, domain)? {
        return Err(format_err!(
            "Unable to create user '{}' as the given domain '{}' does not exist!",
            username,
            domain
        ));
    }

    let pass = query_for_password().unwrap_or_else(|| process::exit(1));
    let pass = hash(&PasswordScheme::Sha512Crypt, &pass).unwrap();

    let a = NewAccount {
        username,
        domain,
        password: &pass,
        quota: Some(quota),
        enabled: Some(enabled),
        sendonly: Some(sendonly),
    };

    Account::create(&conn, a)?;

    println!("User account '{}@{}' has been added!", username, domain);

    Ok(())
}

fn remove(matches: &ArgMatches, conn: DatabaseConnection, domain: Option<&str>) -> Result<()> {
    let username = matches.value_of("USER").unwrap();
    let domain = domain.ok_or_else(|| format_err!("{}", DOMAIN_MISSING))?;
    let verbose = matches.is_present("verbose");
    let force = matches.is_present("force");
    let m = format!(
        "Shall the user account '{}@{}' and all associated aliases be removed? (y/N):",
        username, domain
    );

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
            Alias::delete(&conn, &alias)?;
        }
    }

    Account::delete(&conn, &acc)?;

    println!(
        "User account '{}@{}' and all its aliases has been removed!",
        username, domain
    );

    Ok(())
}

fn password(matches: &ArgMatches, conn: DatabaseConnection, domain: Option<&str>) -> Result<()> {
    let username = matches.value_of("USER").unwrap();
    let domain = domain.ok_or_else(|| format_err!("{}", DOMAIN_MISSING))?;

    let mut acc = Account::get(&conn, username, domain)?;

    let user = format!("{}@{}", username, domain);
    println!("Set password for '{}'", user);
    let pass = query_for_password().unwrap_or_else(|| process::exit(1));
    let pass = hash(&PasswordScheme::Sha512Crypt, &pass).unwrap();

    acc.password = pass;
    Account::save(&conn, &acc)?;

    println!("Password has been changed for {}@{}!", username, domain);
    Ok(())
}

fn edit(matches: &ArgMatches, conn: DatabaseConnection, domain: Option<&str>) -> Result<()> {
    let username = matches.value_of("USER").unwrap();
    let domain = domain.ok_or_else(|| format_err!("{}", DOMAIN_MISSING))?;

    let mut acc = Account::get(&conn, username, domain)?;

    if matches.is_present("enable") {
        acc.enabled = Some(true);
    }
    if matches.is_present("disable") {
        acc.enabled = Some(false);
    }
    if matches.is_present("sendonly") {
        acc.sendonly = Some(true);
    }
    if matches.is_present("sendreceive") {
        acc.sendonly = Some(false);
    }

    if matches.is_present("quota") {
        let quota = value_t!(matches.value_of("quota"), i32).unwrap_or_else(|e| {
            eprintln!("Argument 'quota' has to be >= 0");
            e.exit()
        });
        acc.quota = Some(quota);
    }

    Account::save(&conn, &acc)?;

    println!("Account updated: {}", acc);

    Ok(())
}

pub fn dispatch(matches: &ArgMatches) -> Result<()> {
    dotenv().ok();

    let default_domain = env::var("DEFAULT_DOMAIN");
    let mut domain = if let Some(domain) = default_domain.as_ref().ok() {
        Some(domain.as_str())
    } else {
        None
    };
    if let Some(d) = matches.value_of("domain") {
        domain = Some(d)
    }

    let conn = establish_connection();

    match matches.subcommand() {
        ("show", Some(m)) => show(m, conn, domain),
        ("add", Some(m)) => add(m, conn, domain),
        ("remove", Some(m)) => remove(m, conn, domain),
        ("password", Some(m)) => password(m, conn, domain),
        ("edit", Some(m)) => edit(m, conn, domain),
        _ => show(matches, conn, domain),
    }
}

pub fn get_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("user")
        .about("User management for the vmail database")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("domain")
                .value_name("domain")
                .long("domain")
                .short("d")
                .help("Domain to use for the user subcommands, default can be specified in '.env' file. E.g. mydomain.tld"))
        .subcommand(SubCommand::with_name("show")
                    .about("Show user account(s)")
                    .arg(Arg::with_name("USER")
                            .help("Name of the user account which should be shown"))
                    .arg(Arg::with_name("verbose")
                            .long("verbose")
                            .short("v")
                            .help("Verbose output (include aliases)"))
                    )
        .subcommand(SubCommand::with_name("add")
                    .about("Add a user to the database")
                    .alias("create")
                    .alias("new")
                    .arg(Arg::with_name("USER")
                            .required(true)
                            .help("Name of the user which should be added, without domain name, e.g. 'newuser1'"))
                    .arg(Arg::with_name("disabled")
                            .long("disabled")
                            .short("d")
                            .help("Disable the user, just add it to the database"))
                    .arg(Arg::with_name("sendonly")
                            .long("send-only")
                            .short("s")
                            .help("Allow the new user only to send email but not receive any."))
                    .arg(Arg::with_name("quota")
                            .value_name("quota")
                            .long("quota")
                            .short("q")
                            .default_value("0")
                            .help("Quota for user account in MB (Megabyte), default is 0 which is unlimited"))
                    )
        .subcommand(SubCommand::with_name("remove")
                    .about("Remove a user from the database, will also delete all aliases for the user")
                    .alias("rm")
                    .alias("delete")
                    .arg(Arg::with_name("force")
                            .long("force")
                            .short("f")
                            .help("Force the deleting the given user"))
                    .arg(Arg::with_name("verbose")
                            .long("verbose")
                            .short("v")
                            .help("Verbose output what has been deleted"))
                    .arg(Arg::with_name("USER")
                            .required(true)
                            .help("User which should be removed"))
                    )
        .subcommand(SubCommand::with_name("password")
                    .about("Change the password for given user")
                    .alias("pw")
                    .arg(Arg::with_name("USER")
                            .help("The user name which should be edited")
                            .required(true))
                    )
        .subcommand(SubCommand::with_name("edit")
                    .about("Edit a user account entry")
                    .alias("change")
                    .alias("update")
                    .arg(Arg::with_name("USER")
                            .help("The user name which should be edited")
                            .required(true))
                    .arg(Arg::with_name("disable")
                            .long("disable")
                            .short("d")
                            .conflicts_with("enable")
                            .help("Disable given user"))
                    .arg(Arg::with_name("enable")
                            .long("enable")
                            .short("e")
                            .conflicts_with("disable")
                            .help("Enable given user"))
                    .arg(Arg::with_name("sendonly")
                            .long("send-only")
                            .short("s")
                            .conflicts_with("sendreceive")
                            .help("Allow user only to send"))
                    .arg(Arg::with_name("sendreceive")
                            .long("send-receive")
                            .short("r")
                            .conflicts_with("sendonly")
                            .help("Allow user to send and receive"))
                    .arg(Arg::with_name("quota")
                            .value_name("quota")
                            .long("quota")
                            .short("q")
                            .help("Quota for user account in MB (Megabyte), 0 is unlimited"))
                    )
}
