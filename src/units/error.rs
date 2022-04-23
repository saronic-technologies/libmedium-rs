use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
};

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Error converting values or strings into usable unit types.
#[derive(Debug)]
pub enum Error {
    /// Error which can be returned from reading a raw sensor value.
    RawConversion {
        /// The string that cannot be converted.
        raw: String,
    },

    /// Value to convert into unit type is invalid.
    InvalidValue {
        /// The invalid value
        value: f64,
    },
}

impl Error {
    pub(crate) fn raw_conversion(raw: impl Into<String>) -> Self {
        Self::RawConversion { raw: raw.into() }
    }

    pub(crate) fn invalid_value(value: impl Into<f64>) -> Self {
        Self::InvalidValue {
            value: value.into(),
        }
    }
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RawConversion { raw } => write!(f, "Invalid raw string: {}", raw),
            Self::InvalidValue { value } => {
                write!(f, "Value to convert: {}, is invalid", value)
            }
        }
    }
}
