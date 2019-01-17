use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeywordFilter {
    pub limit: i64,
    pub sort_asc: bool,
}
