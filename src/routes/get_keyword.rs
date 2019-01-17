use actix_web::{self, HttpRequest, HttpResponse, AsyncResponder, FutureResponse};
use crate::db::{DbMessage, DbResult};
use crate::types::AppState;
use futures::future::Future;
use log::error;

pub fn get_keyword((params, req): (actix_web::Path<String>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
	req.state().db.send(DbMessage::SelectByKeyword(params.clone()))
		.from_err()
		.and_then(|res| {
			match res {
				Ok(key_pair) => {
					match key_pair.expect("Not sure if this expect makes sense for GET") {
						DbResult::One(ref pair) => Ok(HttpResponse::Ok().json(pair)),
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
