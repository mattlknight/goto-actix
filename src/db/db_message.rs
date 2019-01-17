use ::actix::prelude::*;
use actix_web::*;
// use actix::{Message};
// use crate::models::KeywordPair;
use crate::db;
use crate::types::{StringError, KeywordPair, KeywordFilter};
// use crate::schema::keywords;
// use serde_derive::{Serialize, Deserialize};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};
use std::mem;

#[derive(Debug)]
pub enum DbMessage<'a> {
    SelectByKeyword(String),
    SelectMany(KeywordFilter, &'a mut Vec<KeywordPair>),
    Insert(KeywordPair),
    Update(String, KeywordPair),
    DeleteByKeyword(KeywordPair),
}

impl<'a> Message for DbMessage<'a> {
    type Result = Result<Option<KeywordPair>, Error>;
}

impl<'a> Handler<DbMessage<'a>> for db::DbCon {
    type Result = Result<Option<KeywordPair>, Error>;

    fn handle(&mut self, mut msg: DbMessage<'a>, _: &mut Self::Context) -> Self::Result {
        use crate::schema::keywords::dsl::*;

        match msg {
            DbMessage::SelectByKeyword(ref req_keyword) => {
                let key_pair = keywords
                    .select((keyword, url))
                    .filter(keyword.eq(req_keyword))
                    .first::<KeywordPair>(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;
                Ok(Some(key_pair))
            },
            DbMessage::SelectMany(ref keyword_filter, ref mut keyword_vec) => {
                let records = match keyword_filter.sort_asc {
                    true => {
                        keywords.select((keyword, url))
                                .order(keyword.asc())
                                .limit(keyword_filter.limit)
                                .load::<KeywordPair>(&self.0)
                                .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?
                    },
                    false => {
                        keywords.select((keyword, url))
                                .order(keyword.desc())
                                .limit(keyword_filter.limit)
                                .load::<KeywordPair>(&self.0)
                                .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?
                    }
                };
                mem::replace(*keyword_vec, records);
                Ok(None)
            },
            DbMessage::Insert(keyword_pair) => {
                diesel::insert_into(keywords)
                    .values(keyword_pair)
                    .execute(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;

                let key_pair = keywords
                    .select((keyword, url))
                    .filter(keyword.eq(&keyword_pair.keyword))
                    .first::<KeywordPair>(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;
                Ok(Some(key_pair))
            },
            DbMessage::Update(ref old_keyword, ref keyword_pair) => {
                let key_pair = diesel::update(keywords.filter(keyword.eq(&old_keyword)))
                    .set((keyword.eq(&keyword_pair.keyword), url.eq(&keyword_pair.url)))
                    .returning((keyword, url))
                    .get_result(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;
                Ok(Some(key_pair))
            },
            DbMessage::DeleteByKeyword(ref keyword_pair) => {
                diesel::delete(keywords.filter(keyword.eq(&keyword_pair.keyword)))
                    .execute(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;
                Ok(None)
            },
        }
    }
}
