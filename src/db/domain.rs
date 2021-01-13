#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::domains;
use diesel::prelude::*;
use std::fmt;

use super::database::DatabaseConnection;
use super::result::{Result, VmailError};

use diesel::{Insertable, Queryable};

use failure::bail;

#[derive(Queryable, PartialEq, Debug)]
pub struct Domain {
    pub id: i32,
    pub domain: String,
}

#[derive(Insertable)]
#[table_name = "domains"]
pub struct NewDomain {
    pub domain: String,
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.domain)
    }
}

impl Domain {
    pub fn all(conn: &DatabaseConnection) -> Result<Vec<Domain>> {
        use super::schema::domains::dsl::*;

        let r = domains.load::<Domain>(conn)?;
        Ok(r)
    }

    pub fn get(conn: &DatabaseConnection, name: &str) -> Result<Domain> {
        use super::schema::domains::dsl::*;

        let mut r = domains
            .filter(domain.eq(name))
            .limit(1)
            .load::<Domain>(conn)?;

        if let Some(r) = r.pop() {
            return Ok(r);
        }

        bail!(VmailError::NotFound(format!("Domain {}", name)));
    }

    pub fn exsits(conn: &DatabaseConnection, name: &str) -> Result<bool> {
        use super::schema::domains::dsl::*;
        use diesel::dsl::exists;
        use diesel::select;

        let r = select(exists(domains.filter(domain.eq(name)))).get_result(conn)?;
        Ok(r)
    }

    /// returns number of rows inserted
    pub fn create(conn: &DatabaseConnection, domain: NewDomain) -> Result<usize> {
        use diesel::insert_into;

        let n = insert_into(domains::table).values(&domain).execute(conn)?;
        Ok(n)
    }

    /// returns number of rows deleted
    pub fn delete(conn: &DatabaseConnection, d: &Domain) -> Result<usize> {
        use super::schema::domains::dsl::*;
        use diesel::delete;

        let n = delete(domains.find(d.id)).execute(conn)?;
        Ok(n)
    }
}
