use crate::schema::keywords;
use serde_derive::{Serialize, Deserialize};

#[derive(Insertable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[table_name="keywords"]
pub struct KeywordPair {
    pub keyword: String,
    pub url: String
}
