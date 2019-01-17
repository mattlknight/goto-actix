use diesel::Connection;
use diesel::pg::PgConnection;
use std::env;

mod print_keywords;
mod db_con;
mod db_message;

pub use print_keywords::print_keywords;
pub use db_con::DbCon;
pub use db_message::DbMessage;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL env var must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
