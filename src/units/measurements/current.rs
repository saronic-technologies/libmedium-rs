use crate::units::{Raw, RawError, RawSensorResult};

use ::measurements;

/// Struct that represents an electrical current.
pub use measurements::Current;

use std::borrow::Cow;

impl Raw for Current {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Current::from_milliamperes)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_milliamperes().round().to_string())
    }
}
