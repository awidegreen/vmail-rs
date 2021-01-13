#![allow(proc_macro_derive_resolution_fallback)]
use super::domain::Domain;
use super::schema::accounts;
use diesel::prelude::*;
use std::fmt;

use super::database::DatabaseConnection;
use super::result::{Result, VmailError};

use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use failure::bail;

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
    pub fn get(conn: &DatabaseConnection, name: &str, domain_: &str) -> Result<Account> {
        use super::schema::accounts::dsl::*;

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

    pub fn all_by_username(conn: &DatabaseConnection, name: &str) -> Result<Vec<Account>> {
        use super::schema::accounts::dsl::*;

        let r = accounts
            .filter(username.eq(name))
            .limit(1)
            .load::<Account>(conn)?;

        if !r.is_empty() {
            return Ok(r);
        }

        bail!(VmailError::NotFound(format!("Account username: {}", name)));
    }

    pub fn all_by_domain(conn: &DatabaseConnection, d: &Domain) -> Result<Vec<Account>> {
        use super::schema::accounts::dsl::*;

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

    pub fn all(conn: &DatabaseConnection) -> Result<Vec<Account>> {
        use super::schema::accounts::dsl::*;
        let r = accounts.load::<Account>(conn)?;
        Ok(r)
    }

    /// returns number of rows inserted
    pub fn create(conn: &DatabaseConnection, account: NewAccount) -> Result<usize> {
        let n = diesel::insert_into(accounts::table)
            .values(account)
            .execute(conn)?;
        Ok(n)
    }

    /// returns number of rows deleted
    pub fn delete(conn: &DatabaseConnection, account: &Account) -> Result<usize> {
        use super::schema::accounts::dsl::*;

        let n = diesel::delete(accounts.find(account.id)).execute(conn)?;
        Ok(n)
    }

    pub fn exsits(conn: &DatabaseConnection, name: &str, domain_name: &str) -> bool {
        use super::schema::accounts::dsl::*;
        use diesel::dsl::exists;
        use diesel::select;

        let r = select(exists(
            accounts
                .filter(username.eq(name))
                .filter(domain.eq(domain_name)),
        ))
        .get_result(conn);
        if let Ok(v) = r {
            v
        } else {
            false
        }
    }

    pub fn save(conn: &DatabaseConnection, account: &Account) -> Result<usize> {
        let n = diesel::update(account).set(account).execute(conn)?;
        Ok(n)
    }
}
