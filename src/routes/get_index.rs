use actix_web::{HttpRequest, HttpResponse};

pub fn get_index(_req: &HttpRequest) -> HttpResponse {
    println!("Get Index");
    let body = include_bytes!("../../static/dist/index.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .header("text", "html")
        .body(body.as_ref())
}
