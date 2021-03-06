use actix_web::{self, HttpRequest, Result, fs::NamedFile};
use crate::types::AppState;
use log::debug;

pub fn get_swagger_yml(_req: &HttpRequest<AppState>) -> Result<NamedFile> {
    debug!("Get Swagger Yml");
    Ok(NamedFile::open("./src/static/swagger.yml")?)
}
