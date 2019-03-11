#![allow(proc_macro_derive_resolution_fallback)]

use account::Account;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use schema::aliases;
use std::fmt;

use result::{Result, VmailError};

#[derive(Queryable, PartialEq, Debug)]
pub struct Alias {
    pub id: i32,
    pub source_username: Option<String>,
    pub source_domain: String,
    pub destination_username: String,
    pub destination_domain: String,
    pub enabled: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "aliases"]
pub struct NewAlias {
    pub source_username: Option<String>,
    pub source_domain: String,
    pub destination_username: String,
    pub destination_domain: String,
    pub enabled: Option<bool>,
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let enabled = match self.enabled {
            Some(v) => v,
            _ => false,
        };
        let username = match self.source_username {
            Some(ref u) => u,
            None => "%",
        };

        write!(
            f,
            "{}@{} => {}@{} (enabled: {})",
            username,
            self.source_domain,
            self.destination_username,
            self.destination_domain,
            enabled
        )
    }
}

impl NewAlias {
    pub fn name(mut self, name: &str) -> Self {
        let name = if name == "%" {
            None
        } else {
            Some(String::from(name))
        };

        self.source_username = name;
        self
    }

    pub fn domain(mut self, domain: &str) -> Self {
        self.source_domain = String::from(domain);
        self.destination_domain = String::from(domain);
        self
    }

    pub fn to_domain(mut self, domain: &str) -> Self {
        self.destination_domain = String::from(domain);
        self
    }

    pub fn to_user(mut self, username: &str) -> Self {
        self.destination_username = String::from(username);
        self
    }

    pub fn enable(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}

impl Alias {
    pub fn source_username(&self) -> String {
        let s = match self.source_username {
            Some(ref u) => u,
            None => "%",
        };
        String::from(s)
    }

    pub fn with_address(username: &str, domain: &str) -> NewAlias {
        Alias::new().domain(domain).name(username)
    }

    pub fn new() -> NewAlias {
        NewAlias {
            source_username: Some(String::new()),
            source_domain: String::new(),
            destination_username: String::new(),
            destination_domain: String::new(),
            enabled: Some(false),
        }
    }

    pub fn get(conn: &MysqlConnection, name: &str, domain: &str) -> Result<Vec<Alias>> {
        use schema::aliases::dsl::*;

        let r = if name == "%" {
            aliases
                .filter(source_username.is_null())
                .filter(source_domain.eq(domain))
                .load::<Alias>(conn)?
        } else {
            aliases
                .filter(source_username.eq(name))
                .filter(source_domain.eq(domain))
                .load::<Alias>(conn)?
        };
        Ok(r)
    }

    pub fn all(conn: &MysqlConnection) -> Result<Vec<Alias>> {
        use schema::aliases::dsl::*;

        let r = aliases.load::<Alias>(conn)?;
        Ok(r)
    }

    pub fn all_by_dest_account(conn: &MysqlConnection, account: &Account) -> Result<Vec<Alias>> {
        use schema::aliases::dsl::*;

        let r = aliases
            .filter(destination_username.eq(&account.username))
            .filter(destination_domain.eq(&account.domain))
            .load::<Alias>(conn)?;

        if !r.is_empty() {
            return Ok(r);
        }

        bail!(VmailError::NotFound(format!(
            "Alias destination_username: {}, destination_domain: {}",
            account.username, account.domain
        )));
    }

    pub fn delete(conn: &MysqlConnection, alias: &Alias) -> Result<usize> {
        use diesel::delete;
        use schema::aliases::dsl::*;

        let n = delete(aliases.find(alias.id)).execute(conn)?;
        Ok(n)
    }

    pub fn create(conn: &MysqlConnection, alias: &NewAlias) -> Result<usize> {
        use diesel;
        use diesel::insert_into;
        use schema::aliases;

        match insert_into(aliases::table).values(alias).execute(conn) {
            Err(diesel::result::Error::DatabaseError(e, d)) => match e {
                diesel::result::DatabaseErrorKind::ForeignKeyViolation => Err(format_err!(
                    "Either the referred domain or user account does not exist."
                )),
                other => Err(format_err!(
                    "Other database error {:?}, details '{:?}'",
                    other,
                    d
                )),
            },
            Err(e) => Err(format_err!("Other error: {}", e)),
            Ok(v) => Ok(v),
        }
    }

    pub fn exsits(conn: &MysqlConnection, name: &str, domain_name: &str) -> bool {
        use diesel::dsl::exists;
        use diesel::select;
        use schema::aliases::dsl::*;

        let r = select(exists(
            aliases
                .filter(source_username.eq(name))
                .filter(source_domain.eq(domain_name)),
        ))
        .get_result(conn);
        if let Ok(v) = r {
            v
        } else {
            false
        }
    }
}
