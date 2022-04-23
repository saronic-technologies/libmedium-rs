use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    num::ParseIntError,
};

#[cfg(feature = "uom_units")]
use std::num::ParseFloatError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Error converting values or strings into usable unit types.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    /// Error which can be returned from reading a raw sensor value.
    RawConversion {
        /// The string that cannot be converted.
        raw: String,
    },

    /// Error parsing string to integer.
    Parsing { source: ParseIntError },

    /// Value to convert into unit type is invalid.
    InvalidValue {
        /// The invalid value
        value: f64,
    },

    /// Error parsing string to float.
    #[cfg(feature = "uom_units")]
    ParsingFloat { source: ParseFloatError },
}

impl Error {
    pub(crate) fn raw_conversion(raw: impl Into<String>) -> Self {
        Self::RawConversion { raw: raw.into() }
    }

    pub(crate) fn parsing(source: ParseIntError) -> Self {
        Self::Parsing { source }
    }

    pub(crate) fn invalid_value(value: impl Into<f64>) -> Self {
        Self::InvalidValue {
            value: value.into(),
        }
    }

    #[cfg(feature = "uom_units")]
    pub(crate) fn parsing_float(source: ParseFloatError) -> Self {
        Self::ParsingFloat { source }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::RawConversion { .. } => None,
            Error::Parsing { source } => Some(source),
            Error::InvalidValue { .. } => None,
            #[cfg(feature = "uom_units")]
            Error::ParsingFloat { source } => Some(source),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RawConversion { raw } => write!(f, "Invalid raw string: {}", raw),
            Self::Parsing { .. } => {
                write!(f, "Error parsing string to integer")
            }
            Self::InvalidValue { value } => {
                write!(f, "Invalid value to convert: {}", value)
            }
            #[cfg(feature = "uom_units")]
            Self::ParsingFloat { .. } => {
                write!(f, "Error parsing string to float")
            }
        }
    }
}
