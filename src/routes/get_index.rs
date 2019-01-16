use actix_web::{HttpRequest, HttpResponse};
use crate::types::AppState;

// pub fn get_index(_req: &HttpRequest<State>) -> Box<dyn Future<Item=HttpResponse, Error=Error>> {
pub fn get_index(_req: &HttpRequest<AppState>) -> HttpResponse {
    println!("Get Index");
    let body = include_bytes!("../../static/dist/index.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .header("text", "html")
        .body(body.as_ref())
}
