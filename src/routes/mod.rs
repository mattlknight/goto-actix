
mod get_favicon;
mod get_index;
mod get_error;
mod get_keyword;
mod get_keywords;
// mod add_keyword;
mod put_keyword;
mod post_keyword;
mod delete_keyword;
mod redirect_keyword;

pub use get_favicon::favicon;
pub use get_index::get_index;
pub use get_error::get_error;
pub use get_keyword::get_keyword;
pub use get_keywords::get_keywords;
// pub use add_keyword::add_keyword;
pub use put_keyword::put_keyword;
pub use post_keyword::post_keyword;
pub use delete_keyword::delete_keyword;
pub use redirect_keyword::redirect_keyword;
