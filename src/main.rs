#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

pub mod schema;
pub mod models;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use lazy_static::lazy_static;
use rocket::http::{ContentType, Status};
use rocket::request::Form;
use rocket::response::{Redirect, Response};
use rocket::http::RawStr;
use rocket_contrib::databases::diesel as rkt_diesel;
// use rocket_contrib::serve::StaticFiles;
use std::env;
use std::io::Cursor;


lazy_static!{
    static ref content_icon:ContentType = ContentType::new("image", "x-icon");
    static ref content_png:ContentType = ContentType::new("image", "png");
}

#[database("pg_db")]
struct DbConn(rkt_diesel::PgConnection);

#[derive(FromForm)]
struct FormKeyword {
    #[form(field = "inputKeyword")]
    keyword: String,
    #[form(field = "inputUrl")]
    url: String,
}

#[get("/favicon.ico")]
fn favicon() -> Response<'static> {
    let fav_icon = include_bytes!("../static/dist/favicon.ico");
    let response = Response::build()
        .status(Status::Ok)
        .raw_header("image", "x-icon")
        .sized_body(Cursor::new(fav_icon.as_ref()))
        .finalize();
    response
}

// #[get("/icon.png")]
// fn icon() -> Response<'static> {
//     let fav_icon = include_bytes!("../static/dist/icon.png");
//     let response = Response::build()
//         .status(Status::Ok)
//         .raw_header("image", "png")
//         .sized_body(Cursor::new(fav_icon.as_ref()))
//         .finalize();
//     response
// }

#[get("/")]
fn index() -> Response<'static> {
    let body = include_bytes!("../static/dist/index.html");
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::HTML)
        .sized_body(Cursor::new(body.as_ref()))
        .finalize();
    response
}

#[get("/error.html")]
fn error() -> Response<'static> {
    let body = include_bytes!("../static/dist/error.html");
    let response = Response::build()
        .status(Status::Ok)
        .header(ContentType::HTML)
        .sized_body(Cursor::new(body.as_ref()))
        .finalize();
    response
}

#[get("/<req_keyword>")]
fn get_keyword(req_keyword: &RawStr) -> Redirect {
    println!("Requested keyword {}!", req_keyword);

    let connection = establish_connection();

    use crate::schema::keywords::dsl::*;
    let records = keywords.filter(keyword.eq(req_keyword.to_string()))
        .limit(1)
        .load::<Keyword>(&connection);
    match records {
        Ok(records) => {
            match records.first() {
                Some(record) => {
                    Redirect::to(format!("{}", record.url))
                },
                None => {
                    Redirect::to("/error.html")
                }
            }
        },
        Err(err) => {
            println!("Error: {:?}", err);
            Redirect::to("/error.html")
        },
    }
}

#[post("/", data = "<form_keyword>")]
fn add_keyword<'a>(form_keyword: Option<Form<FormKeyword>>) -> String {
    match form_keyword {
        Some(form_keyword) => {
            println!("  Passed keyword: {} URL: {}", form_keyword.keyword, form_keyword.url);
            let connection = establish_connection();
            match create_keyword(&connection, &form_keyword.keyword, &form_keyword.url) {
                Ok(result) => format!("Successfully saved {:?}", result),
                Err(err) => format!("{:?}", err),
            }
        },
        None => "Invalid post request".to_owned()
    }

}

fn main() {
    dotenv().ok();
    let connection = establish_connection();
    print_keywords(connection);
    rocket::ignite()
        .mount("/", routes![
            index,
            error,
            favicon,
            // icon,
            add_keyword,
            get_keyword
        ])
        // .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static/dist")))
        // .mount("/", StaticFiles::from("/static"))
        // .mount("/", routes![get_keyword])
        .launch();
}

pub fn print_keywords(connection: diesel::PgConnection) {
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

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Keyword, NewKeyword};

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
