#![allow(proc_macro_derive_resolution_fallback)]
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use domain::Domain;
use schema::accounts;
use std::fmt;

use result::{Result, VmailError};

#[derive(Identifiable, AsChangeset, Queryable, PartialEq, Debug)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub domain: String,
    pub password: String,
    pub quota: Option<i32>,
    pub enabled: Option<bool>,
    pub sendonly: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub domain: &'a str,
    pub password: &'a str,
    pub quota: Option<i32>,
    pub enabled: Option<bool>,
    pub sendonly: Option<bool>,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let enabled = match self.enabled {
            Some(v) => v,
            _ => false,
        };

        let sendonly = match self.sendonly {
            Some(v) => v,
            _ => false,
        };

        let quota = match self.quota {
            Some(v) => v,
            _ => 0,
        };

        write!(
            f,
            "{}@{}\nenabled: {}\nQuota: {}\nsend only: {}",
            self.username, self.domain, enabled, quota, sendonly
        )
    }
}

impl Account {
    pub fn get(conn: &MysqlConnection, name: &str, domain_: &str) -> Result<Account> {
        use schema::accounts::dsl::*;

        let mut r = accounts
            .filter(username.eq(name))
            .filter(domain.eq(domain_))
            .limit(1)
            .load::<Account>(conn)?;

        if let Some(r) = r.pop() {
            return Ok(r);
        }

        bail!(VmailError::NotFound(format!(
            "Account: {}@{}",
            name, domain_
        )));
    }

    pub fn all_by_username(conn: &MysqlConnection, name: &str) -> Result<Vec<Account>> {
        use schema::accounts::dsl::*;

        let r = accounts
            .filter(username.eq(name))
            .limit(1)
            .load::<Account>(conn)?;

        if !r.is_empty() {
            return Ok(r);
        }

        bail!(VmailError::NotFound(format!("Account username: {}", name)));
    }

    pub fn all_by_domain(conn: &MysqlConnection, d: &Domain) -> Result<Vec<Account>> {
        use schema::accounts::dsl::*;

        let r = accounts
            .filter(domain.eq(&d.domain))
            .load::<Account>(conn)?;

        if !r.is_empty() {
            return Ok(r);
        }

        bail!(VmailError::NotFound(format!(
            "Accounts domain: {}",
            &d.domain
        )));
    }

    pub fn all(conn: &MysqlConnection) -> Result<Vec<Account>> {
        use schema::accounts::dsl::*;
        let r = accounts.load::<Account>(conn)?;
        Ok(r)
    }

    /// returns number of rows inserted
    pub fn create(conn: &MysqlConnection, account: NewAccount) -> Result<usize> {
        use schema::accounts;

        let n = diesel::insert_into(accounts::table)
            .values(account)
            .execute(conn)?;
        Ok(n)
    }

    /// returns number of rows deleted
    pub fn delete(conn: &MysqlConnection, account: &Account) -> Result<usize> {
        use schema::accounts::dsl::*;

        let n = diesel::delete(accounts.find(account.id)).execute(conn)?;
        Ok(n)
    }

    pub fn exsits(conn: &MysqlConnection, name: &str, domain_name: &str) -> bool {
        use diesel::dsl::exists;
        use diesel::select;
        use schema::accounts::dsl::*;

        let r = select(exists(
            accounts
                .filter(username.eq(name))
                .filter(domain.eq(domain_name)),
        )).get_result(conn);
        if let Ok(v) = r {
            v
        } else {
            false
        }
    }

    pub fn save(conn: &MysqlConnection, account: &Account) -> Result<usize> {
        let n = diesel::update(account).set(account).execute(conn)?;
        Ok(n)
    }
}
