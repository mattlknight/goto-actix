use crate::models::Keyword;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel::pg::PgConnection;
use log::{log_enabled, debug, Level};

pub fn print_keywords(connection: PgConnection) {
    if log_enabled!(Level::Debug) {
        use crate::schema::keywords::dsl::*;
        let records = keywords
            .order(keyword.asc())
            .load::<Keyword>(&connection)
            .expect("Error loading posts");

        debug!("Displaying {} DB Keyword entries", records.len());
        for record in records {
            debug!("{} <=> {}", record.keyword, record.url);
        }
    }
}
