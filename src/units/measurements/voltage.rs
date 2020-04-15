use crate::units::{Raw, RawError, RawSensorResult};

use ::measurements;

/// Struct that represents an electrical voltage.
pub use measurements::Voltage;

use std::borrow::Cow;

impl Raw for Voltage {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Voltage::from_millivolts)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_millivolts().round().to_string())
    }
}
