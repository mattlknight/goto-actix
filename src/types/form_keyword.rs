use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct FormKeyword {
    pub keyword: String,
    pub url: String,
}
