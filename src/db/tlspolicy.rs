#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::tlspolicies;

use diesel::{Insertable, Queryable};
use diesel_derive_enum::DbEnum;

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
