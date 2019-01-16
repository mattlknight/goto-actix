use actix_web::{self, error, Json};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use crate::models::KeywordPair;
use crate::db;
use crate::types::StringError;

pub fn get_keyword(req_keyword: actix_web::Path<String>) -> actix_web::Result<Json<KeywordPair>> {
// pub fn get_keyword(req_keyword: actix_web::Path<String>) -> impl Responder {
    use crate::schema::keywords::dsl::*;
    println!("Get Keyword");

    println!("Requested keyword {}!", req_keyword);

    let connection = db::establish_connection();

    let result = keywords
            .filter(keyword.eq(req_keyword.as_ref()))
            .select((keyword, url))
            .first::<KeywordPair>(&connection)
            // .find(req_keyword)
            // .filter(keyword.eq(*req_keyword))
            // .first::<KeywordPair>(&connection)
            // .load::<KeywordPair>(&connection)
            .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;

    // let result = result
    //                 .first()
    //                 .ok_or(error::ErrorBadRequest(StringError::new("No results found".into())))?;

    // let result = result.first().ok_or(StringError::new("No results found".into())?;
    Ok(Json(result))
}
