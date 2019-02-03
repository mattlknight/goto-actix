use actix_web::{self, HttpRequest, HttpResponse};
use crate::types::AppState;
use log::debug;

pub fn get_swagger_yml(_req: &HttpRequest<AppState>) -> HttpResponse {
    debug!("Get Swagger Yml");
    let body = include_bytes!("../static/swagger.yml");
    HttpResponse::Ok()
        .content_type("application/x-yaml")
        .header("application", "x-yaml")
        .body(body.as_ref())
}
