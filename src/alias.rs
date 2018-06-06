use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::schema::aliases;
use std::fmt;

use error::Result;

#[derive(Queryable, PartialEq, Debug)]
pub struct Alias {
    pub id: i32,
    pub source_username: String,
    pub source_domain: String,
    pub destination_username: String,
    pub destination_domain: String,
    pub enabled: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "aliases"]
pub struct NewAlias {
    pub source_username: String,
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
        write!(f, "{}@{} ==> {}@{}   ({})",
               self.source_username,
               self.source_domain,
               self.destination_username,
               self.source_domain,
               enabled)
    }
}

impl NewAlias {
    pub fn name(mut self, name: &str) -> Self {
        self.source_username = String::from(name);
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

    pub fn enable(mut self) -> Self {
        self.enabled = Some(true);
        self
    }
}

impl Alias {
    pub fn with_name(name: &str) -> NewAlias {
        Alias::new().name(name)
    }
    pub fn with_address(username: &str, domain: &str) -> NewAlias {
        Alias::new().domain(domain).name(username)
    }

    pub fn new() -> NewAlias {
        NewAlias {
            source_username: String::new(),
            source_domain: String::new(),
            destination_username: String::new(),
            destination_domain: String::new(),
            enabled: Some(false),
        }
    }

    pub fn all(conn: &MysqlConnection) -> Vec<Alias> {
        use schema::aliases::dsl::*;

        aliases.load::<Alias>(conn).expect("Unable to load aliases")
    }

    pub fn delete(conn: &MysqlConnection, alias_id: i32) {
        use schema::aliases::dsl::*;
        use diesel::delete;

        delete(aliases.find(alias_id))
            .execute(conn)
            .expect("Failed to delete alias");
    }

    pub fn create(conn: &MysqlConnection, alias: &NewAlias) -> Result<usize>{
        use schema::aliases;
        use diesel::insert_into;
        use diesel;

        match insert_into(aliases::table).values(alias).execute(conn) {
            Err(diesel::result::Error::DatabaseError(e, d)) => {
                match e {
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation =>
                        Err(format_err!("Either the referred domain or user account does not exist.")),
                    other => Err(format_err!("Other database error {:?}, details '{:?}'", other, d))
                }
            },
            Err(e) => Err(format_err!("Other error: {}", e)),
            Ok(v) => Ok(v)
        }
    }
}
