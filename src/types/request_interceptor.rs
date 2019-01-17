use actix_web::{HttpRequest, HttpResponse, HttpMessage, Result};
use actix_web::middleware::{Middleware, Started, Response};
use log::*;

pub struct RequestInterceptor;

impl<S> Middleware<S> for RequestInterceptor {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let method = req.method();
        let path = req.path();
        let headers = req.headers();
        warn!("{:?}", method);
        warn!("{:?}", path);
        warn!("{:?}", headers);
        Ok(Started::Done)
    }

    fn response(&self, _req: &HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
        Ok(Response::Done(resp))
    }
}
