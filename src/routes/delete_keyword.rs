use actix_web::{self, HttpRequest, HttpResponse, AsyncResponder, FutureResponse};
use crate::db::DbMessage;
use crate::types::AppState;
use futures::future::Future;
use log::error;

pub fn delete_keyword((params, req): (actix_web::Path<String>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
	req.state().db.send(DbMessage::DeleteByKeyword(params.clone()))
		.from_err()
		.and_then(|res| {
			match res {
				Ok(_) => Ok(HttpResponse::Ok().into()),
				Err(err) => {
					error!("{}", err);
					Ok(HttpResponse::InternalServerError().into())
				}
			}
		})
		.responder()
}
