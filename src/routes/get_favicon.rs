use actix_web::{HttpRequest, HttpResponse};

pub fn favicon(_req: &HttpRequest) -> HttpResponse {
    println!("Get FavIcon");
    let fav_icon = include_bytes!("../../static/dist/favicon.ico");
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .header("image", "x-icon")
        .body(fav_icon.as_ref())
}
