use diesel::Connection;
use diesel::pg::PgConnection;
use std::env;

mod create_keyword;
mod print_keywords;

pub use create_keyword::create_keyword;
pub use print_keywords::print_keywords;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
