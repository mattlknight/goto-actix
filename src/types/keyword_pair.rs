// use ::actix::prelude::*;
// use actix_web::*;
// use actix::{Message};
// use crate::models::KeywordPair;
// use crate::db;
// use crate::types::StringError;
use crate::schema::keywords;
use serde_derive::{Serialize, Deserialize};
// use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[table_name="keywords"]
pub struct KeywordPair {
    pub keyword: String,
    pub url: String
}
