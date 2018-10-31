use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use vmail_lib::account::Account;
use vmail_lib::alias::Alias;
use vmail_lib::result::Result;
use vmail_lib::{establish_connection, DatabaseConnection};

//------------------------------------------------------------------------------

fn show(matches: &ArgMatches, conn: DatabaseConnection) -> Result<()> {
    let user = matches.value_of("DEST_USER");
    let domain = matches.value_of("DEST_DOMAIN");

    if let Some(user) = user {
        let domain = domain.unwrap();
        let acc = Account::get(&conn, user, domain)?;
        println!("Filter for destination account: '{}@{}'", user, domain);
        for a in Alias::all_by_dest_account(&conn, &acc)? {
            println!("{}", a)
        }
    } else {
        let all = Alias::all(&conn)?;
        for a in all {
            println!("{}", a)
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn add(matches: &ArgMatches, conn: DatabaseConnection) -> Result<()> {
    let enabled = !matches.is_present("disabled");
    let name = matches.value_of("USER").unwrap();
    let domain = matches.value_of("DOMAIN").unwrap();
    let dest_name = matches.value_of("DEST_USER").unwrap();
    let dest_domain = matches.value_of("DEST_DOMAIN").unwrap_or_else(|| domain);

    let a = Alias::with_address(name, domain)
        .to_domain(dest_domain)
        .to_user(dest_name)
        .enable(enabled);

    Alias::create(&conn, &a)?;

    println!(
        "Alias '{}@{}' for '{}@{}' has been added (enabled: {})!",
        name, domain, dest_name, dest_domain, enabled
    );

    Ok(())
}

//------------------------------------------------------------------------------

fn remove(matches: &ArgMatches, conn: DatabaseConnection) -> Result<()> {
    let user = matches.value_of("USER").unwrap();
    let domain = matches.value_of("DOMAIN").unwrap();

    let alias = Alias::get(&conn, user, domain)?;
    Alias::delete(&conn, &alias)?;

    println!("Alias '{}@{}' has been deleted!", user, domain);

    Ok(())
}

//------------------------------------------------------------------------------

pub fn dispatch(matches: &ArgMatches) -> Result<()> {
    let conn = establish_connection();

    match matches.subcommand() {
        ("add", Some(m)) => add(m, conn),
        ("remove", Some(m)) => remove(m, conn),
        ("show", Some(m)) => show(m, conn),
        _ => show(matches, conn),
    }
}

//------------------------------------------------------------------------------

pub fn get_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("alias")
        .about("Manage aliases for the vmail database")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("show")
                .about("Show aliases for user")
                .aliases(&["list", "ls"])
                .arg(
                    Arg::with_name("DEST_USER")
                        .requires("DEST_DOMAIN")
                        .help("username to filter for"),
                ).arg(Arg::with_name("DEST_DOMAIN").help("domain to filter for")),
        ).subcommand(
            SubCommand::with_name("add")
                .about("Add an alias to an existing user account")
                .aliases(&["create", "new"])
                .arg(
                    Arg::with_name("USER")
                        .required(true)
                        .help("The username for the alias"),
                ).arg(
                    Arg::with_name("DOMAIN")
                        .required(true)
                        .help("Existing domain for the alias"),
                ).arg(
                    Arg::with_name("DEST_USER")
                        .required(true)
                        .help("Existing user account"),
                ).arg(
                    Arg::with_name("DEST_DOMAIN")
                        .help("If not specified, this will assume to DOMAIN value"),
                ).arg(
                    Arg::with_name("disabled")
                        .long("disabled")
                        .short("d")
                        .help("Set alias to disabled"),
                ),
        ).subcommand(
            SubCommand::with_name("remove")
                .about("Remove an alias from the database")
                .alias("rm")
                .alias("delete")
                .arg(Arg::with_name("USER").required(true).help(""))
                .arg(
                    Arg::with_name("DOMAIN")
                        .required(true)
                        .help("Existing domain"),
                ),
        )
}
