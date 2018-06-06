use tlspolicy::PolicyEnumMapping;

table! {
    accounts (id) {
        id -> Integer,
        username -> Varchar,
        domain -> Varchar,
        password -> Varchar,
        quota -> Nullable<Integer>,
        enabled -> Nullable<Bool>,
        sendonly -> Nullable<Bool>,
    }
}

table! {
    aliases (id) {
        id -> Integer,
        source_username -> Varchar,
        source_domain -> Varchar,
        destination_username -> Varchar,
        destination_domain -> Varchar,
        enabled -> Nullable<Bool>,
    }
}

table! {
    domains (id) {
        id -> Integer,
        domain -> Varchar,
    }
}

table! {
    use diesel::sql_types::{Integer, Varchar, Nullable};
    use super::PolicyEnumMapping;
    tlspolicies (id) {
        id -> Integer,
        domain -> Varchar,
        policy -> PolicyEnumMapping,
        params -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(accounts, aliases, domains, tlspolicies,);
