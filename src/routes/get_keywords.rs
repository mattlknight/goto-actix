// use actix_web::{self, FromRequest, Json, HttpRequest, HttpResponse, Responder};
use actix_web::{self, Json, error};
// use actix_web::http::header;
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use crate::models::KeywordPair;
use crate::types::{KeywordFilter, StringError};
use crate::db;

pub fn get_keywords(keyword_filter: Json<KeywordFilter>) -> actix_web::Result<Json<Vec<KeywordPair>>> {
    use crate::schema::keywords::dsl::*;
    // use crate::schema::keywords;

    println!("Get Keywords");

    let connection = db::establish_connection();

    let records = match keyword_filter.sort_asc {
        true => {
            keywords.select((keyword, url))
                    .order(keyword.asc())
                    .limit(keyword_filter.limit)
                    .load::<KeywordPair>(&connection)
        },
        false => {
            keywords.select((keyword, url))
                    .order(keyword.desc())
                    .limit(keyword_filter.limit)
                    .load::<KeywordPair>(&connection)
        }
    };
    Ok(Json(records.map_err(|e| error::ErrorBadRequest(StringError::from(e)))?))
}
