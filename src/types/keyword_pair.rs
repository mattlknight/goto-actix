use ::actix::prelude::*;
use actix_web::*;
// use actix::{Message};
// use crate::models::KeywordPair;
use crate::db;
use crate::types::StringError;
use crate::schema::keywords;
use serde_derive::{Serialize, Deserialize};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[table_name="keywords"]
pub struct KeywordPair {
    pub keyword: String,
    pub url: String
}

impl Message for KeywordPair {
    type Result = Result<KeywordPair, Error>;
}

impl Handler<KeywordPair> for db::DbCon {
    type Result = Result<KeywordPair, Error>;

    fn handle(&mut self, msg: KeywordPair, _: &mut Self::Context) -> Self::Result {
        use crate::schema::keywords::dsl::*;

        diesel::insert_into(keywords)
            .values(&msg)
            .execute(&self.0)
            .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;

        let key_pair = keywords
            .select((keyword, url))
            .filter(keyword.eq(&msg.keyword))
            .first::<KeywordPair>(&self.0)
            .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;

        Ok(key_pair)
    }
}
