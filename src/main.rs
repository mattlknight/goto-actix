#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate rocket;

pub mod schema;
pub mod models;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use rocket::request::Form;
use rocket_contrib::serve::StaticFiles;
use std::env;

use self::models::*;


#[derive(FromForm)]
struct KeywordPair {
    #[form(field = "inputKeyword")]
    keyword: String,
    #[form(field = "inputUrl")]
    url: String,
}

#[post("/", data = "<keyword_pair>")]
fn index<'a>(keyword_pair: Option<Form<KeywordPair>>) -> String {
    match keyword_pair {
        Some(keyword_pair) => {
            format!("You passed keyword: {} URL: {}", keyword_pair.keyword, keyword_pair.url)
        },
        None => "Invalid post request".to_owned()
    }

}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        // .mount("/", StaticFiles::from("/static"))
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .launch();
        main2();
}

fn main2() {
    use crate::schema::keywords::dsl::*;

    let connection = establish_connection();
    // let results = keywords.filter(published.eq(true))
    let results = keywords
        .limit(5)
        .load::<Keyword>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
