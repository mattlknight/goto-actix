use futures::future::{Future, ok};
use actix_web::{HttpRequest, HttpResponse, HttpMessage, AsyncResponder};
use crate::db;
use crate::types;

pub fn add_keyword(req: &HttpRequest) -> Box<dyn Future<Item=HttpResponse, Error=actix_web::error::Error>> {
    println!("Add keyword");
    req.urlencoded::<types::FormKeyword>()
        .from_err()
        .and_then(|data| {
            println!("POST:  Passed keyword: {} URL: {}", data.keyword, data.url);
            let connection = db::establish_connection();
            match db::create_keyword(&connection, &data.keyword, &data.url) {
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
