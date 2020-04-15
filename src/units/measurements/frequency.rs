use crate::units::{Raw, RawError, RawSensorResult};

use ::measurements;

/// Struct that represents a frequency.
pub use measurements::Frequency;

impl Raw for Frequency {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Frequency::from_hertz)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_hertz().round().to_string()
    }
}

