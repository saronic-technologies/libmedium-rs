use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
};

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Error which can be returned from reading a raw sensor value.
#[derive(Debug)]
pub struct Error {
    /// The string that can not be converted.
    raw: String,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid raw string: {}", &self.raw)
    }
}

impl<T: Into<String>> From<T> for Error {
    fn from(raw: T) -> Self {
        Error { raw: raw.into() }
    }
}
