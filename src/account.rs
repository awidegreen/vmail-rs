use diesel;
use schema::accounts;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use std::fmt;

#[derive(Queryable, PartialEq, Debug)]
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

        write!(f, "Name: {}\nDomain: {}\nenabled: {}\nQuota: {}\nsend only: {}",
               self.username,
               self.domain,
               enabled,
               quota,
               sendonly)
    }
}

impl Account {
    pub fn all(conn: &MysqlConnection) -> Vec<Account> {
        use schema::accounts::dsl::*;
        accounts
            .load::<Account>(conn)
            .expect("Coduld not load accounts.")
    }

    pub fn create(conn: &MysqlConnection, acc: NewAccount) {
        use schema::accounts;

        diesel::insert_into(accounts::table)
            .values(&acc)
            .execute(conn)
            .expect("Unable to insert new account");
    }

    pub fn delete(conn: &MysqlConnection, account_id: i32)  {
        use schema::accounts::dsl::*;

        diesel::delete(accounts.find(account_id))
            .execute(conn)
            .expect("Failed to delete account");
    }

    pub fn exsits(conn: &MysqlConnection, name: &str, domain_name: &str) -> bool {
        use schema::accounts::dsl::*;
        use diesel::dsl::exists;
        use diesel::select;

        let r = select(exists(accounts
                .filter(username.eq(name))
                .filter(domain.eq(domain_name))))
            .get_result(conn);
        if let Ok(v) = r { v } else { false }
    }
}
