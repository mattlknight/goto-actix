use actix_web::{self, HttpRequest, HttpResponse, Json, AsyncResponder, FutureResponse};
// use actix_web::{self, HttpRequest, HttpResponse, HttpMessage, FromRequest, Json, AsyncResponder, FutureResponse};
use futures::future::Future;
use crate::types::{AppState, KeywordPair};
use log::error;
// use std::ops::Deref;

pub fn put_keyword((data, _params, req): (Json<KeywordPair>, actix_web::Path<String>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
	let data = data.clone();
	req.state().db.send(KeywordPair{keyword: data.keyword, url: data.url})
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
