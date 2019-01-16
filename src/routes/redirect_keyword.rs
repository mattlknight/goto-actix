use actix_web::{self, HttpResponse};
use actix_web::http::header;
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use crate::models::Keyword;
use crate::db;

pub fn redirect_keyword(req_keyword: actix_web::Path<String>) -> HttpResponse {
    println!("Get Keyword");

    println!("Requested keyword {}!", req_keyword);

    let connection = db::establish_connection();

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
