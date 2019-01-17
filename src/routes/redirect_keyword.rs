use actix_web::{self, HttpRequest, HttpResponse, AsyncResponder, FutureResponse};
use actix_web::http::header;
use crate::db::{DbMessage, DbResult};
use crate::types::AppState;
use futures::future::Future;
use log::error;

pub fn redirect_keyword((params, req): (actix_web::Path<String>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
	req.state().db.send(DbMessage::SelectByKeyword(params.clone()))
		.from_err()
		.and_then(|res| {
			match res {
				Ok(key_pair) => {
					match key_pair.expect("Not sure if this expect makes sense for REDIRECT") {
						DbResult::One(ref pair) => {
                            Ok(HttpResponse::TemporaryRedirect()
                                .header(header::LOCATION, pair.url.clone())
                                .finish())
                    },
						DbResult::Many(_) => unreachable!()
					}
				},
				Err(err) => {
					error!("{}", err);
					Ok(HttpResponse::InternalServerError().finish())
				}
			}
		})
		.responder()
}

// use actix_web::{self, HttpResponse};
// use actix_web::http::header;
// use crate::db;
// use crate::models::Keyword;
// use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
// use log::debug;

// pub fn redirect_keyword(req_keyword: actix_web::Path<String>) -> HttpResponse {
//     debug!("Get Keyword");
//
//     debug!("Requested keyword {}!", req_keyword);
//
//     let connection = db::establish_connection();
//
//     use crate::schema::keywords::dsl::*;
//     let records = keywords.filter(keyword.eq(req_keyword.to_string()))
//         .limit(1)
//         .load::<Keyword>(&connection);
//     match records {
//         Ok(records) => {
//             match records.first() {
//                 Some(ref record) => {
//                     HttpResponse::TemporaryRedirect()
//                         .header(header::LOCATION, record.url.clone())
//                         .finish()
//                 },
//                 None => {
//                     HttpResponse::TemporaryRedirect()
//                         .header(header::LOCATION, "/error.html")
//                         .finish()
//                 }
//             }
//         },
//         Err(err) => {
//             debug!("Error: {:?}", err);
//             HttpResponse::TemporaryRedirect()
//                 .header(header::LOCATION, "/error.html")
//                 .finish()
//         },
//     }
// }
