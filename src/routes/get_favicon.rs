use actix_web::{HttpRequest, HttpResponse};
use crate::types::AppState;

pub fn favicon(_req: &HttpRequest<AppState>) -> HttpResponse {
    println!("Get FavIcon");
    let fav_icon = include_bytes!("../../static/dist/favicon.ico");
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .header("image", "x-icon")
        .body(fav_icon.as_ref())
}
