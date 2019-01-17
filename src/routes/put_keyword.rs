use actix_web::{self, HttpRequest, HttpResponse, Json, AsyncResponder, FutureResponse};
use crate::db::{DbMessage, DbResult};
use crate::types::{AppState, KeywordPair};
use futures::future::Future;
use log::error;
use diesel::result::Error as DieselError;

pub fn put_keyword((data, params, req): (Json<KeywordPair>, actix_web::Path<String>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
	req.state().db.send(DbMessage::Update(params.clone(), data.clone()))
		.from_err()
		.and_then(|res| {
			match res {
				Ok(key_pair) => {
					match key_pair.expect("Should always return some for PUT") {
						DbResult::One(ref pair) => Ok(HttpResponse::Ok().json(pair)),
						DbResult::Many(_) => unreachable!()
					}
				},
				Err(DieselError::NotFound) => Ok(HttpResponse::NotFound().finish()),
				Err(err) => {
					error!("{}", err);
					unimplemented!()
				},
			}
		})
		.responder()
}
