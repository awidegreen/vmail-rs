use diesel::connection::Connection;
use diesel::mysql::MysqlConnection;

pub type DatabaseConnection = MysqlConnection;

pub fn connect(database_url: &str) -> DatabaseConnection {

    DatabaseConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
