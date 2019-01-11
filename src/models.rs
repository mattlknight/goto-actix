use chrono::NaiveDateTime;
use super::schema::keywords;

#[derive(Insertable, Debug)]
#[table_name="keywords"]
pub struct NewKeyword<'a> {
    pub keyword: &'a str,
    pub url: &'a str,
}

#[derive(Queryable, Debug)]
pub struct Keyword {
    pub id: i32,
    pub keyword: String,
    pub url: String,
    pub created_on: NaiveDateTime,
    pub modified_on: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct KeywordTrace {
    pub id: i32,
    pub keyword_id: i32,
    pub accessed_on: NaiveDateTime,
}
