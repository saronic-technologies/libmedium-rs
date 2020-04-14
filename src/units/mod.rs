//! Units used in this library.

mod humidity;

#[cfg(not(feature = "measurements_units"))]
mod native;

#[cfg(feature = "measurements_units")]
mod measurements;

pub use humidity::Humidity;

#[cfg(not(feature = "measurements_units"))]
pub use native::*;

#[cfg(feature = "measurements_units")]
pub use self::measurements::*;

use snafu::Snafu;

use std::time::Duration;

pub(crate) type RawSensorResult<T> = std::result::Result<T, RawError>;

/// Error which can be returned from reading a raw sensor value.
#[derive(Snafu, Debug)]
pub enum RawError {
    /// The read string is invalid and can not be converted into the desired value type.
    #[snafu(display("Invalid raw string: {}", raw))]
    InvalidRawString {
        /// The string that can not be converted.
        raw: String,
    },
}

impl<T: AsRef<str>> From<T> for RawError {
    fn from(raw: T) -> Self {
        RawError::InvalidRawString {
            raw: raw.as_ref().to_string(),
        }
    }
}

/// Trait that needs to be implemented by all types that raw sensor strings should be converted into.
pub trait Raw: Sized {
    /// Converts a raw sensor string into a usable type.
    fn from_raw(raw: &str) -> RawSensorResult<Self>;

    /// Converts self into a writable raw sensor string.
    fn to_raw(&self) -> String;
}

impl Raw for bool {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        match raw.trim() {
            "1" => Ok(true),
            "0" => Ok(false),
            other => Err(RawError::from(other)),
        }
    }

    fn to_raw(&self) -> String {
        match self {
            true => String::from("1"),
            false => String::from("0"),
        }
    }
}

impl Raw for String {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        Ok(raw.trim().to_string())
    }

    fn to_raw(&self) -> String {
        self.clone()
    }
}

impl Raw for Duration {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u64>()
            .map(Duration::from_millis)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_millis().to_string()
    }
}
