#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
extern crate dotenv;
// #[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_contrib;

pub mod schema;
pub mod models;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use futures::future::{Future, ok};
use serde_derive::Deserialize;
use std::env;
// use std::io::Cursor;
use actix_web::{server, App, HttpRequest, HttpResponse, HttpMessage, AsyncResponder};
// use actix_web::Responder;
use actix_web::http::{header, Method};
use self::models::{Keyword, NewKeyword};

fn main() {
    dotenv().ok();
    let connection = establish_connection();
    print_keywords(connection);

    server::new(|| {
        App::new()
            .resource("/", |r| r.method(Method::GET).f(get_index))
            .resource("/", |r| r.method(Method::POST).f(add_keyword))
            .resource("/error.html", |r| r.f(get_error))
            .resource("/favicon.ico", |r| r.f(favicon))
            .resource("/{keyword}", |r| r.method(Method::GET).f(get_keyword))
    })
    .bind("0.0.0.0:80")
    .expect("Can not bind to 0.0.0.0:80")
    .run();
}

fn get_index(_req: &HttpRequest) -> HttpResponse {
    let body = include_bytes!("../static/dist/index.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .header("text", "html")
        .body(body.as_ref())
}

fn get_error(_req: &HttpRequest) -> HttpResponse {
    let body = include_bytes!("../static/dist/error.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .header("text", "html")
        .body(body.as_ref())
}

fn favicon(_req: &HttpRequest) -> HttpResponse {
    let fav_icon = include_bytes!("../static/dist/favicon.ico");
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .header("image", "x-icon")
        .body(fav_icon.as_ref())
}

fn get_keyword(req: &HttpRequest) -> HttpResponse {
    let req_keyword = req.match_info()
                    .get("keyword")
                    .expect("Failed to parse keyword from route match info");
    println!("Requested keyword {}!", req_keyword);

    let connection = establish_connection();

    use crate::schema::keywords::dsl::*;
    let records = keywords.filter(keyword.eq(req_keyword.to_string()))
        .limit(1)
        .load::<Keyword>(&connection);
    match records {
        Ok(records) => {
            match records.first() {
                Some(ref record) => {
                    HttpResponse::TemporaryRedirect()
                        .header(header::LOCATION, record.url.clone())
                        .finish()
                },
                None => {
                    HttpResponse::TemporaryRedirect()
                        .header(header::LOCATION, "/error.html")
                        .finish()
                }
            }
        },
        Err(err) => {
            println!("Error: {:?}", err);
            HttpResponse::TemporaryRedirect()
                .header(header::LOCATION, "/error.html")
                .finish()
        },
    }
}

#[derive(Deserialize)]
struct FormKeyword {
    keyword: String,
    url: String,
}

fn add_keyword(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=actix_web::error::Error>> {
    req.urlencoded::<FormKeyword>()
        .from_err()
        .and_then(|data| {
            println!("  Passed keyword: {} URL: {}", data.keyword, data.url);
            let connection = establish_connection();
            match create_keyword(&connection, &data.keyword, &data.url) {
                Ok(result) => { ok(HttpResponse::Ok()
                                .body(format!("Successfully saved {:?}", result))
                                .into())
                },
                Err(err) => { ok(HttpResponse::Ok()
                                .body(format!("{:?}", err))
                                .into())
                }
            }
        })
        .responder()
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
