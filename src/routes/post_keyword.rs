use actix_web::{self, HttpRequest, HttpResponse, Json, AsyncResponder, FutureResponse};
use crate::db::{DbMessage, DbResult};
use crate::types::{AppState, KeywordPair};
use futures::future::Future;
use log::error;

pub fn post_keyword((data, req): (Json<KeywordPair>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
	req.state().db.send(DbMessage::Insert(data.clone()))
		.from_err()
		.and_then(|res| {
			match res {
				Ok(key_pair) => {
					match key_pair.expect("Should always return some for POST") {
						DbResult::One(ref pair) => Ok(HttpResponse::Ok().json(pair)),
						DbResult::Many(_) => unreachable!()
					}
				},
				Err(err) => {
					error!("{}", err);
					Ok(HttpResponse::InternalServerError().into())
				}
			}
		})
		.responder()
}
