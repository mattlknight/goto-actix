use chrono::NaiveDateTime;
use crate::schema::keywords;
use serde_derive::Serialize;

#[derive(Insertable, Queryable, Debug, Serialize)]
#[table_name="keywords"]
pub struct KeywordPair {
    pub keyword: String,
    pub url: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Keyword {
    pub id: i32,
    pub keyword: String,
    pub url: String,
    pub created_on: NaiveDateTime,
    pub modified_on: NaiveDateTime,
}

// #[derive(Insertable, Queryable, Debug, Serialize)]
// #[table_name="keywords"]
// pub struct NewKeyword<'a> {
//     pub keyword: &'a str,
//     pub url: &'a str,
// }

// #[derive(Queryable, Debug)]
// pub struct KeywordTrace {
//     pub id: i32,
//     pub keyword_id: i32,
//     pub accessed_on: NaiveDateTime,
// }
