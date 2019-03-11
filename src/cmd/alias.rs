use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use vmail_lib::account::Account;
use vmail_lib::alias::Alias;
use vmail_lib::result::Result;
use vmail_lib::{establish_connection, DatabaseConnection};

use utils;

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

    if Account::exsits(&conn, name, domain) {
        return Err(format_err!(
            "Unable to add alias: '{}@{}' exists as a user-account!",
            name,
            domain
        ));
    }

    if !Account::exsits(&conn, dest_name, dest_domain) {
        if !Alias::exsits(&conn, dest_name, dest_domain) {
            return Err(format_err!(
                "Unable to add alias '{}@{}' as destination '{}@{}' does not exists as user account nor as an existing alias!",
                name,
                domain,
                dest_name,
                dest_domain,
            ));
        }
    }

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
    let dest_user = matches.value_of("dest_user");
    let dest_domain = matches.value_of("dest_domain");
    let force = matches.is_present("force");

    let aliases = Alias::get(&conn, user, domain)?;

    for alias in aliases {
        if let Some(dest_user) = dest_user {
            if alias.destination_username != dest_user {
                continue;
            }
        }
        if let Some(dest_domain) = dest_domain {
            if alias.destination_domain != dest_domain {
                continue;
            }
        }

        if !force {
            let m = format!("Shall the alias {} really be removed?", alias);
            if utils::yes_no(&m, utils::YesNoAnswer::NO) == utils::YesNoAnswer::NO {
                continue;
            }
        }
        Alias::delete(&conn, &alias)?;

        println!("Alias {} has been deleted!", alias);
    }

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
                )
                .arg(Arg::with_name("DEST_DOMAIN").help("domain to filter for")),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about(
                    "Add an alias to an existing user account. See USER help for catch-all aliases",
                )
                .aliases(&["create", "new"])
                .arg(
                    Arg::with_name("USER")
                    .required(true)
                    .long_help(
"Username for the alias. If the username is specified as '%'
(percentage) it will act as a catch-all user.",))
                .arg(
                    Arg::with_name("DOMAIN")
                        .required(true)
                        .help("Existing domain for the alias"),
                )
                .arg(
                    Arg::with_name("DEST_USER")
                        .required(true)
                        .help("Existing user account"),
                )
                .arg(
                    Arg::with_name("DEST_DOMAIN")
                        .help("If not specified, this will assume the <DOMAIN> value"),
                )
                .arg(
                    Arg::with_name("disabled")
                        .long("disabled")
                        .short("d")
                        .help("Set alias to disabled"),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove aliases from the database. See <USER> argument help for remove a catch-all user.")
                .alias("rm")
                .alias("delete")
                .arg(
                    Arg::with_name("USER")
                        .required(true)
                        .long_help(
"Username for the alias. For removing a catch-all alias, the '%'
(percentage) should be used as a <USER>."))
                .arg(
                    Arg::with_name("DOMAIN")
                        .required(true)
                        .help("Domain of the alias (need to exist)"),
                )
                .arg(
                    Arg::with_name("dest_user")
                        .value_name("dest_user")
                        .long("destination-user")
                        .short("u")
                        .help("Filter on destination account/user name"),
                )
                .arg(
                    Arg::with_name("dest_domain")
                        .value_name("dest_domain")
                        .long("destination-domain")
                        .short("d")
                        .help("Filter on destination account/user domain"),
                )
                .arg(
                    Arg::with_name("force")
                        .long("force")
                        .short("f")
                        .help("Delete the aliases without confirmation"),
                ),
        )
}
