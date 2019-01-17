use actix_web::{self, HttpRequest, HttpResponse, Json, AsyncResponder, FutureResponse};
use crate::db::{DbMessage, DbResult};
use crate::types::{AppState, KeywordFilter};
use futures::future::Future;
use log::error;

pub fn get_keywords((data, req): (Json<KeywordFilter>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
	req.state().db.send(DbMessage::SelectMany(data.clone()))
		.from_err()
		.and_then(|res| {
			match res {
				Ok(key_pair) => {
					match key_pair.expect("Should always return some for GET MANY") {
						DbResult::Many(ref pairs) => Ok(HttpResponse::Ok().json(pairs)),
						DbResult::One(_) => unreachable!()
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
