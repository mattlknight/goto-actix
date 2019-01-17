use std::borrow::Cow;
use std::fmt;

#[derive(Debug)]
pub struct StringError<'a> {
   msg: Cow<'a, str>
}

impl<'a> From<diesel::result::Error> for StringError<'a> {
    fn from(error: diesel::result::Error) -> Self {
        StringError { msg: error.to_string().into() }
    }
}

impl<'a> fmt::Display for StringError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
