//! Units used in this library.

mod accuracy;
mod error;
mod fan_divisor;
mod humidity;
mod pwm;
mod temp_type;

#[cfg(not(feature = "measurements_units"))]
mod native;

#[cfg(feature = "measurements_units")]
mod measurements;

pub use accuracy::Accuracy;
pub use error::Error;
pub use fan_divisor::FanDivisor;
pub use humidity::Humidity;
pub use pwm::*;
pub use temp_type::TempType;

#[cfg(not(feature = "measurements_units"))]
pub use native::*;

#[cfg(feature = "measurements_units")]
pub use self::measurements::*;

pub(crate) use error::Result;

use std::borrow::Cow;
use std::time::Duration;

/// Trait that needs to be implemented by all types that raw sensor strings should be converted into.
pub trait Raw: Sized {
    /// Converts a raw sensor string into a usable type.
    fn from_raw(raw: &str) -> Result<Self>;

    /// Converts self into a writable raw sensor string.
    fn to_raw(&self) -> Cow<str>;
}

impl Raw for bool {
    fn from_raw(raw: &str) -> Result<Self> {
        match raw.trim() {
            "1" => Ok(true),
            "0" => Ok(false),
            other => Err(Error::from(other)),
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
    fn from_raw(raw: &str) -> Result<Self> {
        Ok(raw.trim().to_string())
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Borrowed(self.as_str())
    }
}

impl Raw for Duration {
    fn from_raw(raw: &str) -> Result<Self> {
        raw.trim()
            .parse::<u64>()
            .map(Duration::from_millis)
            .map_err(|_| Error::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_millis().to_string())
    }
}
