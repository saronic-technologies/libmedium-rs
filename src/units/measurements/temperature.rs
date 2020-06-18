use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use ::measurements;

/// Struct that represents a temperature.
pub use measurements::Temperature;

use std::borrow::Cow;

impl Raw for Temperature {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(|raw| Temperature::from_celsius(raw / 1000.0))
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned((self.as_celsius() * 1000.0).round().to_string())
    }
}
