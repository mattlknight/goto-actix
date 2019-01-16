use actix_web::{self, error, Json};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use log::debug;
use crate::models::KeywordPair;
use crate::db;
use crate::types::StringError;

pub fn get_keyword(req_keyword: actix_web::Path<String>) -> actix_web::Result<Json<KeywordPair>> {
    use crate::schema::keywords::dsl::*;
    debug!("GET::/api/keyword/{}!", req_keyword);

    let connection = db::establish_connection();

    let result = keywords
            .filter(keyword.eq(req_keyword.as_ref()))
            .select((keyword, url))
            .first::<KeywordPair>(&connection)
            .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;

    Ok(Json(result))
}
