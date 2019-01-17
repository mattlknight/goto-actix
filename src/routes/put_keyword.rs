use actix_web::{self, HttpRequest, HttpResponse, Json, AsyncResponder, FutureResponse};
use futures::future::Future;
use crate::types::{AppState, KeywordPair};
use crate::db::DbMessage;
use log::error;

pub fn put_keyword((data, _params, req): (Json<KeywordPair>, actix_web::Path<String>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
	req.state().db.send(DbMessage::Update(data.keyword.clone(), data.clone()))
		.from_err()
		.and_then(|res| {
			match res {
				Ok(user) => Ok(HttpResponse::Ok().json(user)),
				Err(err) => {
					error!("{}", err);
					Ok(HttpResponse::InternalServerError().into())
				}
			}
		})
		.responder()
}
