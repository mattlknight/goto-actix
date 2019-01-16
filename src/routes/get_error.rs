use actix_web::{HttpRequest, HttpResponse};

pub fn get_error(_req: &HttpRequest) -> HttpResponse {
    println!("Get Error");
    let body = include_bytes!("../../static/dist/error.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .header("text", "html")
        .body(body.as_ref())
}
