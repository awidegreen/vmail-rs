use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use schema::domains;
use std::fmt;

use error::Result;

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
    pub fn all(conn: &MysqlConnection) -> Vec<Domain> {
        use schema::domains::dsl::*;

        domains
            .load::<Domain>(conn)
            .expect("Could not load domains.")
    }

    pub fn exsits(conn: &MysqlConnection, name: &str) -> bool {
        use schema::domains::dsl::*;
        use diesel::dsl::exists;
        use diesel::select;

        let r = select(exists(domains.filter(domain.eq(name)))).get_result(conn);
        if let Ok(v) = r { v } else { false }
    }

    pub fn create(conn: &MysqlConnection, domain: NewDomain) {
        use schema::domains;
        use diesel::insert_into;

        insert_into(domains::table)
            .values(&domain)
            .execute(conn)
            .expect("Unable to insert new domain");
    }

    pub fn delete(conn: &MysqlConnection, name: &str)  -> Result<usize> {
        use schema::domains::dsl::*;
        use diesel::delete;

        let n = delete(domains.filter(domain.eq(name))).execute(conn)?;
        Ok(n)
    }
}
