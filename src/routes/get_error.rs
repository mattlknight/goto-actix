use actix_web::{HttpRequest, HttpResponse};
use crate::types::AppState;

pub fn get_error(_req: &HttpRequest<AppState>) -> HttpResponse {
    println!("Get Error");
    let body = include_bytes!("../../static/dist/error.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .header("text", "html")
        .body(body.as_ref())
}
