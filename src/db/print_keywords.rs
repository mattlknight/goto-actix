use crate::models::Keyword;
use diesel::RunQueryDsl;
use diesel::pg::PgConnection;

#[allow(dead_code)]
pub fn print_keywords(connection: PgConnection) {
    use crate::schema::keywords::dsl::*;
    let records = keywords
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
