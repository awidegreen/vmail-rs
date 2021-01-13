use diesel::connection::Connection;

#[cfg(not(feature = "postgres"))]
pub use diesel::mysql::MysqlConnection as DatabaseConnection;
#[cfg(feature = "postgres")]
pub use diesel::pg::PgConnection as DatabaseConnection;

pub fn connect(database_url: &str) -> DatabaseConnection {
    DatabaseConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
