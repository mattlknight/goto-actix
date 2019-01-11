use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Keyword {
    pub row_id: i32,
    pub keyword: String,
    pub url: String,
    pub created_on: NaiveDateTime,
    pub modified_on: NaiveDateTime,
}
