use diesel::pg::PgConnection;
use diesel::{RunQueryDsl};
use crate::models::{Keyword, NewKeyword};

pub fn create_keyword<'a>(conn: &PgConnection, keyword: &'a str, url: &'a str) -> Result<Keyword, diesel::result::Error> {
    use crate::schema::keywords;

    let new_keyword = NewKeyword {
        keyword: keyword,
        url: url,
    };

    diesel::insert_into(keywords::table)
        .values(&new_keyword)
        .get_result(conn)
}
