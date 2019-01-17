use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct KeywordFilter {
    pub limit: i64,
    pub sort_asc: bool,
}
