mod database;
pub use database::Database;

pub type Result<T> = std::result::Result<T, mysql::Error>;