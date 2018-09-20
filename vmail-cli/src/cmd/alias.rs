use clap::ArgMatches;
use vmail_lib::account::Account;
use vmail_lib::alias::Alias;
use vmail_lib::establish_connection;
use vmail_lib::result::Result;

fn show(matches: &ArgMatches) -> Result<()> {
    let user = matches.value_of("DEST_USER");
    let domain = matches.value_of("DEST_DOMAIN");

    let conn = establish_connection();

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

fn add(matches: &ArgMatches) -> Result<()> {
    let enabled = !matches.is_present("disabled");
    let name = matches.value_of("USER").unwrap();
    let domain = matches.value_of("DOMAIN").unwrap();
    let dest_name = matches.value_of("DEST_USER").unwrap();
    let dest_domain = matches.value_of("DEST_DOMAIN").unwrap_or_else(|| domain);

    let conn = establish_connection();

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

fn remove(matches: &ArgMatches) -> Result<()> {
    let user = matches.value_of("USER").unwrap();
    let domain = matches.value_of("DOMAIN").unwrap();

    let conn = establish_connection();
    let alias = Alias::get(&conn, user, domain)?;
    Alias::delete(&conn, alias)?;

    println!("Alias '{}@{}' has been deleted!", user, domain);

    Ok(())
}

pub fn dispatch(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("add", Some(m)) => add(m),
        ("remove", Some(m)) => remove(m),
        ("show", Some(m)) => show(m),
        _ => show(matches),
    }
}
