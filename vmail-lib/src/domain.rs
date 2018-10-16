#![allow(proc_macro_derive_resolution_fallback)]
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use schema::domains;
use std::fmt;

use result::{Result, VmailError};

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
    pub fn all(conn: &MysqlConnection) -> Result<Vec<Domain>> {
        use schema::domains::dsl::*;

        let r = domains.load::<Domain>(conn)?;
        Ok(r)
    }

    pub fn get(conn: &MysqlConnection, name: &str) -> Result<Domain> {
        use schema::domains::dsl::*;

        let mut r = domains
            .filter(domain.eq(name))
            .limit(1)
            .load::<Domain>(conn)?;

        if let Some(r) = r.pop() {
            return Ok(r);
        }

        bail!(VmailError::NotFound(format!("Domain {}", name)));
    }

    pub fn exsits(conn: &MysqlConnection, name: &str) -> Result<bool> {
        use diesel::dsl::exists;
        use diesel::select;
        use schema::domains::dsl::*;

        let r = select(exists(domains.filter(domain.eq(name)))).get_result(conn)?;
        Ok(r)
    }

    /// returns number of rows inserted
    pub fn create(conn: &MysqlConnection, domain: NewDomain) -> Result<usize> {
        use diesel::insert_into;
        use schema::domains;

        let n = insert_into(domains::table).values(&domain).execute(conn)?;
        Ok(n)
    }

    /// returns number of rows deleted
    pub fn delete(conn: &MysqlConnection, d: &Domain) -> Result<usize> {
        use diesel::delete;
        use schema::domains::dsl::*;

        let n = delete(domains.find(d.id)).execute(conn)?;
        Ok(n)
    }
}
