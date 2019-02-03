use actix_web::{self, HttpRequest, HttpResponse};
use crate::types::AppState;
use log::debug;

pub fn get_swagger_index(_req: &HttpRequest<AppState>) -> HttpResponse {
    debug!("Get Swagger Index");
    let body = include_bytes!("../static/swaggerui_index.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .header("text", "html")
        .body(body.as_ref())
}
