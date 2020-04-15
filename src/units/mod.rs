//! Units used in this library.

mod accuracy;
mod fan_divisor;
mod humidity;
mod pwm;
mod temp_type;

#[cfg(not(feature = "measurements_units"))]
mod native;

#[cfg(feature = "measurements_units")]
mod measurements;

pub use accuracy::Accuracy;
pub use fan_divisor::FanDivisor;
pub use humidity::Humidity;
pub use pwm::*;
pub use temp_type::TempType;

#[cfg(not(feature = "measurements_units"))]
pub use native::*;

#[cfg(feature = "measurements_units")]
pub use self::measurements::*;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use std::borrow::Cow;

pub(crate) type RawSensorResult<T> = std::result::Result<T, RawError>;

/// Error which can be returned from reading a raw sensor value.
#[derive(Debug)]
pub struct RawError {
    /// The string that can not be converted.
    raw: String,
}

impl Error for RawError {}

impl Display for RawError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid raw string: {}", &self.raw)
    }
}

impl<T: Into<String>> From<T> for RawError {
    fn from(raw: T) -> Self {
        RawError { raw: raw.into() }
    }
}

/// Trait that needs to be implemented by all types that raw sensor strings should be converted into.
pub trait Raw: Sized {
    /// Converts a raw sensor string into a usable type.
    fn from_raw(raw: &str) -> RawSensorResult<Self>;

    /// Converts self into a writable raw sensor string.
    fn to_raw(&self) -> Cow<str>;
}

impl Raw for bool {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        match raw.trim() {
            "1" => Ok(true),
            "0" => Ok(false),
            other => Err(RawError::from(other)),
        }
    }

    fn to_raw(&self) -> Cow<str> {
        match self {
            true => Cow::Borrowed("1"),
            false => Cow::Borrowed("0"),
        }
    }
}

impl Raw for String {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        Ok(raw.trim().to_string())
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Borrowed(self.as_str())
    }
}

impl Raw for Duration {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u64>()
            .map(Duration::from_millis)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_millis().to_string())
    }
}
