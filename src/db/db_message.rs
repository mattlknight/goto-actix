use actix_web::{error, Error};
use actix_web::actix::{Message, Handler};
use crate::db::{self, DbResult};
use crate::types::{StringError, KeywordPair, KeywordFilter};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

#[derive(Debug)]
pub enum DbMessage {
    SelectByKeyword(String),
    SelectMany(KeywordFilter),
    Insert(KeywordPair),
    Update(String, KeywordPair),
    DeleteByKeyword(String),
}

impl Message for DbMessage {
    type Result = Result<Option<DbResult<KeywordPair>>, Error>;
}

impl Handler<DbMessage> for db::DbCon {
    type Result = Result<Option<DbResult<KeywordPair>>, Error>;

    fn handle(&mut self, msg: DbMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::keywords::dsl::*;

        match msg {
            DbMessage::SelectByKeyword(ref req_keyword) => {
                let key_pair = keywords
                    .select((keyword, url))
                    .filter(keyword.eq(req_keyword))
                    .first::<KeywordPair>(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;
                Ok(Some(DbResult::One(key_pair)))
            },
            DbMessage::SelectMany(ref keyword_filter) => {
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
                Ok(Some(DbResult::Many(records)))
            },
            DbMessage::Insert(keyword_pair) => {
                diesel::insert_into(keywords)
                    .values(&keyword_pair)
                    .execute(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;

                let key_pair = keywords
                    .select((keyword, url))
                    .filter(keyword.eq(&keyword_pair.keyword))
                    .first::<KeywordPair>(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;
                Ok(Some(DbResult::One(key_pair)))
            },
            DbMessage::Update(ref old_keyword, ref keyword_pair) => {
                let key_pair = diesel::update(keywords.filter(keyword.eq(&old_keyword)))
                    .set((keyword.eq(&keyword_pair.keyword), url.eq(&keyword_pair.url)))
                    .returning((keyword, url))
                    .get_result(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;
                Ok(Some(DbResult::One(key_pair)))
            },
            DbMessage::DeleteByKeyword(ref key_name) => {
                diesel::delete(keywords.filter(keyword.eq(&key_name)))
                    .execute(&self.0)
                    .map_err(|e| error::ErrorBadRequest(StringError::from(e)))?;
                Ok(None)
            },
        }
    }
}
