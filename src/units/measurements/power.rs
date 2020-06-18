use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use ::measurements;

/// Struct that represents electrical power.
pub use measurements::Power;

use std::borrow::Cow;

impl Raw for Power {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Power::from_microwatts)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_microwatts().round().to_string())
    }
}
