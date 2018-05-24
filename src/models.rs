use super::schema::accounts;
use super::schema::aliases;
use super::schema::domains;
use super::schema::tlspolicies;

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

#[derive(DbEnum, Debug, PartialEq)]
pub enum PolicyEnum {
    None,
    May,
    Encrypt,
    Dane,
    DaneOnly,
    Fingerprint,
    Verify,
    Secure,
}

#[derive(Queryable, Debug, PartialEq)]
pub struct Tlspolicy {
    pub id: i32,
    pub domain: String,
    pub policy: PolicyEnum,
    pub params: Option<String>,
}

#[derive(Insertable, Debug, PartialEq)]
#[table_name = "tlspolicies"]
pub struct NewTlspolicy {
    pub domain: String,
    pub policy: PolicyEnum,
    pub params: Option<String>,
}
