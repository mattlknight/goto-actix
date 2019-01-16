use diesel::pg::PgConnection;
use diesel::{QueryDsl, RunQueryDsl};
use crate::models::Keyword;

#[allow(dead_code)]
pub fn print_keywords(connection: PgConnection) {
    use crate::schema::keywords::dsl::*;
    // let results = keywords.filter(published.eq(true))
    let records = keywords
        .limit(5)
        .load::<Keyword>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} entries", records.len());
    for record in records {
        println!("{}", record.keyword);
        println!("    {}", record.id);
        println!("    {}", record.url);
        println!("    {}", record.created_on);
        println!("    {}", record.modified_on);
    }
}
